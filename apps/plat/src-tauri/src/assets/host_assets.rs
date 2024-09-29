use std::{collections::HashMap, fs, path::PathBuf};

use daemon::daemon::PluginDaemon;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use super::{daemon_asset::DaemonAsset, template_asset::TemplateAsset};

// 资产树
pub struct HostAssets {
    pub path: PathBuf,
    pub daemons: Mutex<HashMap<String, DaemonAsset>>,
    pub templates: Mutex<HashMap<String, TemplateAsset>>,
}

impl HostAssets {
    pub async fn new_from_scan(app_handle: &AppHandle) -> anyhow::Result<Self> {
        let host_assets_dir = app_handle.path().app_data_dir()?.join("assets");
        let host_assets = HostAssets {
            path: host_assets_dir,
            daemons: Mutex::new(HashMap::new()),
            templates: Mutex::new(HashMap::new()),
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

        let templates_dir = host_assets.path.join("templates");
        if templates_dir.exists() {
            for entry in templates_dir.read_dir()? {
                let template_asset = TemplateAsset::new_from_path(entry?.path()).await?;
                host_assets
                    .templates
                    .lock()
                    .await
                    .insert(template_asset.sha3_256_string.clone(), template_asset);
            }
        }
        host_assets.templates.lock().await.insert(
            "default".to_string(),
            TemplateAsset::new_from_default(app_handle).await?,
        );

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

    pub async fn append_daemon(&self, plugin_daemon: PluginDaemon) -> anyhow::Result<()> {
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
        daemon_asset.up().await?;

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
