use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::RwLock;

use crate::assets::host_assets::HostAssets;

pub struct HostStateInner {
    pub host_assets: RwLock<HostAssets>,
}
pub type HostState<'a> = State<'a, HostStateInner>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteDaemon {
    pub address: String,
}
