use std::path::PathBuf;

use crate::core::plugin::Plugin;
use crate::core::signature_box::SignatureBox;
use base64::prelude::*;
use glob::glob;
use ring::{
    rand,
    signature::{self, KeyPair},
};
use serde::{Deserialize, Serialize};

use super::plugin::PluginRuntime;

#[derive(Serialize, Deserialize)]
pub struct Isolate {
    pub public_key: String,
    private_key: String,

    #[serde(skip)]
    plugins: Vec<Plugin>,
}

impl Isolate {
    pub fn generate() -> Result<Self, String> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = match signature::Ed25519KeyPair::generate_pkcs8(&rng) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let keypair = match signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };
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

    pub async fn init_plugin(&mut self, plugins_dir: PathBuf) {
        // 扫描 plugin_dir 目录下的 plugin.json 文件
        for entry in glob(plugins_dir.join("**/plugin.json").to_str().unwrap())
            .expect("failed to read glob pattern")
        {
            let entry = match entry {
                Ok(value) => value,
                _ => continue,
            };

            let plugin_dir = entry.join("..");
            let plugin = match Plugin::load_by_path(plugin_dir) {
                Ok(value) => value,
                _ => continue,
            };

            let rt_plugin = plugin.clone();
            tokio::spawn(async move {
                let rt = PluginRuntime::from_plugin(rt_plugin)
                    .await
                    .expect("create plugin runtime failed");
                rt.start().await.expect("start plugin failed");
            });

            self.plugins.push(plugin);
        }
    }
}
