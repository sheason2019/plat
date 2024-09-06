use std::{
    collections::HashMap,
    future::Future,
    rc::Rc,
    sync::{Arc, Mutex},
};

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::daemon::PluginDaemon;

#[derive(Clone)]
pub struct ServiceState {
    pub daemon: PluginDaemon,
    pub registed_plugins: Arc<Mutex<HashMap<String, models::RegistedPlugin>>>,
    pub confirm_signature_handler: Arc<ConfirmSignatureHandler>,
}

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

pub type ConfirmSignatureHandler =
    dyn Fn(SignRequest) -> BoxFuture<'static, anyhow::Result<bool>> + Send + Sync + 'static;
