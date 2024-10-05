use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistPluginRequest {
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRequest {
    pub base64_url_data_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub base64_url_data_string: String,
    pub signature: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub success: bool,
}
