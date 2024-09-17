use std::{collections::HashMap, fs, ops::DerefMut, path::PathBuf, sync::Arc};

use daemon::{
    daemon::{PluginDaemon, PluginDaemonVariant},
    service::PluginDaemonService,
};
use tokio::sync::Mutex;

use super::plugin_asset::PluginAsset;

pub struct DaemonAsset {
    pub path: PathBuf,
    pub plugins: Mutex<HashMap<String, PluginAsset>>,
    plugin_daemon: PluginDaemon,
    plugin_daemon_service: Mutex<Option<Arc<PluginDaemonService>>>,
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
                let plugin_asset =
                    PluginAsset::new_from_path(entry?.path(), &daemon_asset.plugin_daemon).await?;
                daemon_asset
                    .plugins
                    .lock()
                    .await
                    .insert(plugin_asset.plugin_config.name.clone(), plugin_asset);
            }
        }

        Ok(daemon_asset)
    }

    pub async fn get_plugin_daemon(&self) -> PluginDaemon {
        match self.plugin_daemon.variant {
            PluginDaemonVariant::Local => self
                .plugin_daemon_service
                .lock()
                .await
                .as_ref()
                .unwrap()
                .plugin_daemon
                .clone(),
            _ => self.plugin_daemon.clone(),
        }
    }

    pub async fn up(&self) -> anyhow::Result<()> {
        match self.plugin_daemon.variant {
            PluginDaemonVariant::Local => (),
            _ => return Ok(()),
        }

        let mut plugin_daemon_service_option = self.plugin_daemon_service.lock().await;
        if plugin_daemon_service_option.is_some() {
            return Ok(());
        }

        let plugin_daemon_service = PluginDaemonService::new(self.plugin_daemon.clone(), 0).await?;
        plugin_daemon_service_option.replace(plugin_daemon_service);

        for plugin in self.plugins.lock().await.values() {
            plugin.up().await?;
        }

        Ok(())
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        for plugin in self.plugins.lock().await.values() {
            plugin.down().await?;
        }

        let mut lock = self.plugin_daemon_service.lock().await;
        if lock.is_some() {
            lock.as_ref().unwrap().stop().await?;
        }

        *lock.deref_mut() = None;

        Ok(())
    }
}
