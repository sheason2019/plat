use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::core::signature_box::SignatureBox;
use anyhow::Context;
use base64::prelude::*;
use glob::glob;
use platx_core::platx::{daemon::PlatXDaemon, PlatX};
use ring::{
    rand,
    signature::{self, KeyPair},
};

pub struct Isolate {
    pub public_key: String,
    pub private_key: String,

    pub daemon: PlatXDaemon,
    pub plugin_handler_map: HashMap<String, PlatX>,
}

impl Isolate {
    pub async fn generate() -> anyhow::Result<Self> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

        let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key_bytes = keypair.public_key().as_ref();

        let public_key = BASE64_URL_SAFE.encode(public_key_bytes);
        let private_key = BASE64_URL_SAFE.encode(pkcs8_bytes);

        let mut daemon = PlatXDaemon::new();
        daemon.start_server().await?;

        Ok(Isolate {
            public_key,
            private_key,
            daemon,
            plugin_handler_map: HashMap::new(),
        })
    }

    pub fn create_sig_box(&self, message: Vec<u8>) -> SignatureBox {
        let pkcs8_bytes = BASE64_URL_SAFE
            .decode(self.private_key.clone())
            .expect("base64 decode failed");

        let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let sig = keypair.sign(&message);

        SignatureBox {
            message: BASE64_URL_SAFE.encode(message),
            public_key: self.public_key.clone(),
            sig: BASE64_URL_SAFE.encode(sig),
        }
    }

    pub async fn init_plugin(&mut self, plugins_dir: PathBuf) -> anyhow::Result<()> {
        // 扫描 plugin_dir 目录下的 plugin.json 文件
        for entry in glob(plugins_dir.join("**/plugin.json").to_str().unwrap())? {
            let entry = entry?;
            let plugin_dir = entry.join("..");

            let mut plugin = PlatX::from_plugin_root(plugin_dir.clone()).context(format!(
                "init plugin by path {} failed",
                plugin_dir.to_str().unwrap()
            ))?;

            let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;

            plugin
                .start_server(tcp_listener, self.daemon.addr.clone())
                .await
                .context("start server failed")?;
            self.plugin_handler_map
                .insert(plugin.registed_plugin.config.name.clone(), plugin);
        }

        Ok(())
    }

    pub async fn uninstall_plugin(&mut self, name: String) -> anyhow::Result<()> {
        // 从 Daemon 服务中移除 Plugin 的注册
        self.daemon.uninstall_plugin(&name)?;

        // 尝试从本机服务中寻找句柄，若存在句柄则停止服务并从文件系统删除 Plugin
        match self.plugin_handler_map.remove(&name) {
            None => return Ok(()),
            Some(plugin) => {
                plugin.stop().await;
                plugin.delete_in_fs()?;
            }
        };

        Ok(())
    }

    pub async fn install_plugin(&mut self, plugin_file_path: PathBuf) -> anyhow::Result<()> {
        let plugin_root = Path::new("data")
            .join(self.public_key.clone())
            .join("plugins");

        let untarer = platx_core::bundler::untarer::Untarer::new(plugin_file_path);
        let plugin_path = untarer.untar_with_plugin_root(plugin_root)?;
        let mut plugin = PlatX::from_plugin_root(plugin_path)?;
        let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        plugin
            .start_server(tcp_listener, self.daemon.addr.clone())
            .await?;

        Ok(())
    }
}
