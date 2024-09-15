use std::path::PathBuf;

use plugin::{models::PluginConfig, PluginService};
use tokio::sync::Mutex;

pub struct PluginAsset {
    path: PathBuf,
    plugin_config: PluginConfig,
    plugin_service: Mutex<Option<PluginService>>,
}
