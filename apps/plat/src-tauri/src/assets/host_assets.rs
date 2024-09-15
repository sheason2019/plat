use std::{collections::HashMap, path::PathBuf};

use super::daemon_asset::DaemonAsset;

// 资产树
pub struct HostAssets {
    path: PathBuf,
    daemons: HashMap<String, DaemonAsset>,
}
