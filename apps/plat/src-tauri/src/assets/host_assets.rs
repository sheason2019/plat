use std::{collections::HashMap, fs, path::PathBuf};

use daemon::daemon::PluginDaemon;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use super::daemon_asset::DaemonAsset;

// 资产树
pub struct HostAssets {
    pub path: PathBuf,
    pub daemons: Mutex<HashMap<String, DaemonAsset>>,
}

impl HostAssets {
    pub async fn new_from_scan(app_handle: &AppHandle) -> anyhow::Result<Self> {
        let host_assets_dir = app_handle.path().app_data_dir()?.join("assets");
        let host_assets = HostAssets {
            path: host_assets_dir,
            daemons: Mutex::new(HashMap::new()),
        };

        if !host_assets.path.exists() {
            fs::create_dir_all(&host_assets.path)?;
            return Ok(host_assets);
        }

        let daemons_dir = host_assets.path.join("daemons");
        if daemons_dir.exists() {
            for entry in daemons_dir.read_dir()? {
                let daemon_asset = DaemonAsset::new_from_path(entry?.path()).await?;
                host_assets.daemons.lock().await.insert(
                    daemon_asset.get_plugin_daemon().await.daemon_key(),
                    daemon_asset,
                );
            }
        }

        Ok(host_assets)
    }

    pub async fn up(&self, app_handle: &AppHandle) -> anyhow::Result<()> {
        for daemon in self.daemons.lock().await.values() {
            daemon.up(app_handle).await?;
        }

        Ok(())
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        for daemon in self.daemons.lock().await.values() {
            daemon.down().await?;
        }

        Ok(())
    }

    pub async fn append_daemon(
        &self,
        app_handle: &AppHandle,
        plugin_daemon: PluginDaemon,
    ) -> anyhow::Result<()> {
        let daemon_dir = self
            .path
            .join("daemons")
            .join(urlencoding::encode(plugin_daemon.daemon_key().as_str()).as_ref());

        if !daemon_dir.exists() {
            fs::create_dir_all(&daemon_dir)?;
        }

        fs::write(
            daemon_dir.join("daemon.json"),
            serde_json::to_string(&plugin_daemon)?,
        )?;

        let daemon_asset = DaemonAsset::new_from_path(daemon_dir).await?;
        daemon_asset.up(app_handle).await?;

        self.daemons
            .lock()
            .await
            .insert(plugin_daemon.daemon_key(), daemon_asset);

        Ok(())
    }

    pub async fn delete_daemon(&self, daemon_key: String) -> anyhow::Result<()> {
        match self.daemons.lock().await.remove(&daemon_key) {
            None => (),
            Some(asset) => {
                asset.down().await?;
                fs::remove_dir_all(asset.path)?;
            }
        }

        Ok(())
    }
}
