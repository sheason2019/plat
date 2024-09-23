use std::{fs, ops::DerefMut, path::PathBuf, sync::Arc};

use daemon::daemon::PluginDaemon;
use plugin::{models::PluginConfig, PluginService};
use tokio::sync::Mutex;

pub struct PluginAsset {
    pub path: PathBuf,
    pub plugin_config: PluginConfig,
    plugin_service: Mutex<Option<Arc<PluginService>>>,
}

impl PluginAsset {
    pub async fn new_from_path(
        path: PathBuf,
        plugin_daemon: &PluginDaemon,
    ) -> anyhow::Result<PluginAsset> {
        let plugin_config_bytes = fs::read(path.join("plugin.json"))?;
        let mut plugin_config: PluginConfig = serde_json::from_slice(&plugin_config_bytes)?;

        plugin_config.daemon_address = Some(plugin_daemon.address.as_ref().unwrap().clone());

        let plugin_asset = PluginAsset {
            path,
            plugin_config,
            plugin_service: Mutex::new(None),
        };

        Ok(plugin_asset)
    }

    pub async fn up(&self) -> anyhow::Result<()> {
        let mut plugin_service_option = self.plugin_service.lock().await;
        if plugin_service_option.is_some() {
            return Ok(());
        }

        let plugin_service =
            PluginService::new(self.path.clone(), self.plugin_config.clone(), 0).await?;
        plugin_service_option.replace(Arc::new(plugin_service));

        Ok(())
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        let mut lock = self.plugin_service.lock().await;
        if lock.is_some() {
            lock.as_ref().unwrap().stop().await;
        }

        *lock.deref_mut() = None;
        Ok(())
    }
}
