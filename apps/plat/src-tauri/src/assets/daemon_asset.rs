use std::{collections::HashMap, fs, path::PathBuf};

use daemon::{daemon::PluginDaemon, service::PluginDaemonService};
use tokio::sync::Mutex;

use super::plugin_asset::PluginAsset;

pub struct DaemonAsset {
    pub path: PathBuf,
    pub plugins: Mutex<HashMap<String, PluginAsset>>,
    pub plugin_daemon: PluginDaemon,
    plugin_daemon_service: Mutex<Option<PluginDaemonService>>,
}

impl DaemonAsset {
    pub async fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        let plugin_daemon_bytes = fs::read(path.join("daemon.json"))?;
        let plugin_daemon: PluginDaemon = serde_json::from_slice(&plugin_daemon_bytes)?;

        let daemon_asset = DaemonAsset {
            path,
            plugin_daemon,
            plugins: Mutex::new(HashMap::new()),
            plugin_daemon_service: Mutex::new(None),
        };

        let plugins_dir = daemon_asset.path.join("plugins");
        if plugins_dir.exists() {
            for entry in plugins_dir.read_dir()? {
                let plugin_asset = PluginAsset::new_from_path(entry?.path()).await?;
                daemon_asset
                    .plugins
                    .lock()
                    .await
                    .insert(plugin_asset.plugin_config.name.clone(), plugin_asset);
            }
        }

        Ok(daemon_asset)
    }

    async fn up(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn down(&self) -> anyhow::Result<()> {
        todo!()
    }
}
