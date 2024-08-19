use std::path::PathBuf;

use crate::core::signature_box::SignatureBox;
use base64::prelude::*;
use glob::glob;
use platx_core::platx::PlatX;
use ring::{
    rand,
    signature::{self, KeyPair},
};

pub struct Isolate {
    pub public_key: String,
    pub private_key: String,

    pub plugins: Vec<PlatX>,
}

impl Isolate {
    pub fn generate() -> anyhow::Result<Self> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

        let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key_bytes = keypair.public_key().as_ref();

        let public_key = BASE64_URL_SAFE.encode(public_key_bytes);
        let private_key = BASE64_URL_SAFE.encode(pkcs8_bytes);

        Ok(Isolate {
            public_key,
            private_key,
            plugins: Vec::new(),
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

            let mut plugin = PlatX::from_path(plugin_dir)?;

            let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;

            plugin.start_server(tcp_listener).await?;

            self.plugins.push(plugin);
        }

        Ok(())
    }
}
