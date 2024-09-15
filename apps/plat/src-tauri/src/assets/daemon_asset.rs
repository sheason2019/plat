use std::{collections::HashMap, path::PathBuf};

use daemon::{daemon::PluginDaemon, service::PluginDaemonService};
use tokio::sync::Mutex;

use super::plugin_asset::PluginAsset;

pub struct DaemonAsset {
    path: PathBuf,
    plugin_daemon: PluginDaemon,
    plugins: HashMap<String, PluginAsset>,
    plugin_daemon_service: Mutex<Option<PluginDaemonService>>,
}
