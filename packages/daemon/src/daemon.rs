use base64::prelude::*;
use ring::{
    rand,
    signature::{self, KeyPair},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginDaemon {
    pub public_key: String,
    private_key: String,
}

impl PluginDaemon {
    pub fn generate(data_root: PathBuf) -> anyhow::Result<Self> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

        let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key_bytes = keypair.public_key().as_ref();

        let public_key = BASE64_URL_SAFE.encode(public_key_bytes);
        let private_key = BASE64_URL_SAFE.encode(pkcs8_bytes);

        let daemon = PluginDaemon {
            public_key,
            private_key,
        };

        daemon.save(data_root)?;

        Ok(daemon)
    }

    pub fn from_directory(daemon_directory: PathBuf) -> anyhow::Result<Self> {
        let file_path = daemon_directory.join("daemon.json");
        let file_bytes = fs::read(file_path)?;

        let daemon: PluginDaemon = serde_json::from_slice(&file_bytes)?;
        Ok(daemon)
    }

    pub fn save(&self, data_root: PathBuf) -> anyhow::Result<()> {
        let file_path = data_root.join(&self.public_key).join("daemon.json");
        let parent_dir = file_path.parent().unwrap();
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        let file_bytes = serde_json::to_string(self)?;
        fs::write(file_path, file_bytes)?;

        Ok(())
    }
}
