use base64::prelude::*;
use ring::signature::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignatureBox {
    pub message: String,
    pub public_key: String,
    pub sig: String,
}

impl SignatureBox {
    pub fn verify(&self) -> Result<(), ring::error::Unspecified> {
        let public_key = signature::UnparsedPublicKey::new(
            &signature::ED25519,
            BASE64_STANDARD.decode(self.public_key.clone()).unwrap(),
        );
        public_key.verify(
            BASE64_STANDARD
                .decode(self.message.clone())
                .unwrap()
                .as_ref(),
            BASE64_STANDARD.decode(self.sig.clone()).unwrap().as_ref(),
        )
    }
}
