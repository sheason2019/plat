use std::{collections::HashMap, fs, path::PathBuf};

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
                    daemon_asset
                        .path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    daemon_asset,
                );
            }
        }

        Ok(host_assets)
    }

    pub async fn up(&self) -> anyhow::Result<()> {
        for daemon in self.daemons.lock().await.values() {
            daemon.up().await?;
        }

        Ok(())
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        for daemon in self.daemons.lock().await.values() {
            daemon.down().await?;
        }

        Ok(())
    }
}
