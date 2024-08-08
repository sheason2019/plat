use crate::core::signature_box::SignatureBox;
use base64::prelude::*;
use ring::{
    rand,
    signature::{self, KeyPair},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Isolate {
    public_key: String,
    private_key: String,
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

        let public_key = BASE64_STANDARD.encode(public_key_bytes);
        let private_key = BASE64_STANDARD.encode(pkcs8_bytes);

        Ok(Isolate {
            public_key,
            private_key,
        })
    }

    pub fn create_sig_box(&self, message: Vec<u8>) -> SignatureBox {
        let pkcs8_bytes = BASE64_STANDARD
            .decode(self.private_key.clone())
            .expect("base64 decode failed");

        let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let sig = keypair.sign(&message);

        SignatureBox {
            message: BASE64_STANDARD.encode(message),
            public_key: self.public_key.clone(),
            sig: BASE64_STANDARD.encode(sig),
        }
    }
}
