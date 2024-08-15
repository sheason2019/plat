use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::core::plugin::Plugin;
use crate::core::signature_box::SignatureBox;
use base64::prelude::*;
use glob::glob;
use ring::{
    rand,
    signature::{self, KeyPair},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Isolate {
    pub public_key: String,
    pub private_key: String,

    pub plugins: Vec<Plugin>,
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

    pub async fn init_plugin(this: Arc<Mutex<Self>>, plugins_dir: PathBuf) {
        let mut recvs: Vec<tokio::sync::mpsc::UnboundedReceiver<()>> = Vec::new();

        // 扫描 plugin_dir 目录下的 plugin.json 文件
        for entry in glob(plugins_dir.join("**/plugin.json").to_str().unwrap())
            .expect("failed to read glob pattern")
        {
            let entry = entry.expect("failed to get entry path");
            let plugin_dir = entry.join("..");

            let plugin = Plugin::load_by_path(plugin_dir).expect(
                format!("failed to load plugin by path {:?}", entry.clone().to_str()).as_ref(),
            );

            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            recvs.push(rx);

            tokio::spawn({
                let isolate = Arc::clone(&this);
                async move {
                    let mut plugin = plugin.clone();
                    let (listener, server) =
                        plugin.create_server().await.expect("start plugin failed");
                    plugin.addr = listener.local_addr().unwrap().to_string();
                    isolate.lock().unwrap().plugins.push(plugin);

                    tx.send(()).unwrap();

                    axum::serve(listener, server).await.unwrap();
                }
            });
        }

        for mut recv in recvs {
            recv.recv().await.unwrap();
        }
    }
}
