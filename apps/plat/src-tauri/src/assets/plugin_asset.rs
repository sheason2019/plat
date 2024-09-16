use std::{fs, path::PathBuf};

use plugin::{models::PluginConfig, PluginService};
use tokio::sync::Mutex;

pub struct PluginAsset {
    pub path: PathBuf,
    pub plugin_config: PluginConfig,
    plugin_service: Mutex<Option<PluginService>>,
}

impl PluginAsset {
    pub async fn new_from_path(path: PathBuf) -> anyhow::Result<PluginAsset> {
        let plugin_config_bytes = fs::read(path.join("plugin.json"))?;
        let plugin_config: PluginConfig = serde_json::from_slice(&plugin_config_bytes)?;

        let plugin_asset = PluginAsset {
            path,
            plugin_config,
            plugin_service: Mutex::new(None),
        };

        Ok(plugin_asset)
    }

    pub async fn up(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        todo!()
    }
}
