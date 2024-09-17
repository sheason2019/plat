use base64::prelude::*;
use ring::{
    rand,
    signature::{self, KeyPair},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PluginDaemonVariant {
    Local,
    Remote,
    Hybrid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginDaemon {
    pub public_key: String,
    pub private_key: String,
    pub password: String,
    pub variant: PluginDaemonVariant,
    pub address: Option<String>,
}

impl PluginDaemon {
    pub const fn default() -> Self {
        PluginDaemon {
            public_key: String::new(),
            private_key: String::new(),
            password: String::new(),
            variant: PluginDaemonVariant::Local,
            address: None,
        }
    }

    pub fn new_random() -> anyhow::Result<Self> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

        let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key_bytes = keypair.public_key().as_ref();

        let public_key = BASE64_URL_SAFE.encode(public_key_bytes);
        let private_key = BASE64_URL_SAFE.encode(pkcs8_bytes);

        let daemon = PluginDaemon {
            public_key,
            private_key,
            password: "".to_string(),
            variant: PluginDaemonVariant::Local,
            address: None,
        };

        Ok(daemon)
    }

    pub fn sign(&self, base64_url_data_string: String) -> anyhow::Result<SignBox> {
        let private_key_bytes = BASE64_URL_SAFE.decode(&self.private_key)?;
        let key_pair = ring::signature::Ed25519KeyPair::from_pkcs8(&private_key_bytes)
            .expect("create keypair failed");

        let data_bytes = BASE64_URL_SAFE.decode(base64_url_data_string)?;
        let sig = key_pair.sign(&data_bytes);

        Ok(SignBox {
            public_key: self.public_key.clone(),
            signature: BASE64_URL_SAFE.encode(sig.as_ref()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignBox {
    pub public_key: String,
    pub signature: String,
}

impl SignBox {
    pub fn verify(&self, base64_url_data_string: String) -> anyhow::Result<bool> {
        let public_key = BASE64_URL_SAFE.decode(&self.public_key)?;
        let data_bytes = BASE64_URL_SAFE.decode(base64_url_data_string)?;
        let signature = BASE64_URL_SAFE.decode(&self.signature)?;
        let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);

        match public_key.verify(&data_bytes, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
