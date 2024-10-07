use serde::{Deserialize, Serialize};
use tauri::State;

use crate::assets::host_assets::HostAssets;

pub struct HostStateInner {
    pub host_assets: HostAssets,
}
pub type HostState<'a> = State<'a, HostStateInner>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteDaemon {
    pub address: String,
    pub password: String,
}
