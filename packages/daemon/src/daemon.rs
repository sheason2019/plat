use anyhow::anyhow;
use base64::prelude::*;
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Daemon {
    pub public_key: String,
    pub private_key: String,
}

impl Daemon {
    pub const fn default() -> Self {
        Daemon {
            public_key: String::new(),
            private_key: String::new(),
        }
    }

    pub fn new_random() -> anyhow::Result<Self> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Ok(Daemon {
            private_key: BASE64_URL_SAFE.encode(signing_key.as_bytes()),
            public_key: BASE64_URL_SAFE.encode(verifying_key.as_bytes()),
        })
    }

    pub fn sign(&self, base64_url_data_string: String) -> anyhow::Result<SignBox> {
        let mut signing_key = SigningKey::from_bytes(
            BASE64_URL_SAFE
                .decode(self.private_key.clone())?
                .as_slice()
                .try_into()?,
        );

        let data_bytes = BASE64_URL_SAFE.decode(base64_url_data_string)?;
        let sig = signing_key.sign(&data_bytes);

        Ok(SignBox {
            public_key: self.public_key.clone(),
            signature: BASE64_URL_SAFE.encode(sig.to_bytes()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignBox {
    pub public_key: String,
    pub signature: String,
}

impl SignBox {
    pub fn verify(&self, base64_url_data_string: String) -> anyhow::Result<()> {
        let verifying_key = VerifyingKey::from_bytes(
            BASE64_URL_SAFE
                .decode(self.public_key.clone())?
                .as_slice()
                .try_into()?,
        )?;

        let signature: Signature =
            Signature::from_slice(&BASE64_URL_SAFE.decode(&self.signature)?)?;

        match verifying_key
            .verify_strict(&BASE64_URL_SAFE.decode(base64_url_data_string)?, &signature)
        {
            Ok(()) => Ok(()),
            Err(_) => Err(anyhow!("签名校验不通过")),
        }
    }
}
