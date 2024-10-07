use std::{collections::HashMap, fs, path::PathBuf};

use daemon::daemon::Daemon;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use crate::typings::RemoteDaemon;

use super::{
    local_daemon_asset::LocalDaemonAsset, remote_daemon_asset::RemoteDaemonAsset,
    template_asset::TemplateAsset,
};

// 资产树
pub struct HostAssets {
    pub path: PathBuf,
    pub local_daemons: Mutex<HashMap<String, LocalDaemonAsset>>,
    pub remote_daemons: Mutex<HashMap<String, RemoteDaemonAsset>>,
    pub templates: Mutex<HashMap<String, TemplateAsset>>,
}

impl HostAssets {
    pub fn empty() -> Self {
        HostAssets {
            path: PathBuf::new(),
            local_daemons: Mutex::new(HashMap::new()),
            remote_daemons: Mutex::new(HashMap::new()),
            templates: Mutex::new(HashMap::new()),
        }
    }

    pub async fn new_from_scan(app_handle: &AppHandle) -> anyhow::Result<Self> {
        let host_assets_dir = app_handle.path().app_data_dir()?.join("assets");
        let host_assets = HostAssets {
            path: host_assets_dir,
            local_daemons: Mutex::new(HashMap::new()),
            remote_daemons: Mutex::new(HashMap::new()),
            templates: Mutex::new(HashMap::new()),
        };

        if !host_assets.path.exists() {
            fs::create_dir_all(&host_assets.path)?;
            return Ok(host_assets);
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

        let local_daemons_dir = host_assets.path.join("daemons").join("local");
        if local_daemons_dir.exists() {
            for entry in local_daemons_dir.read_dir()? {
                let templates_map = host_assets.templates.lock().await;
                let local_daemon_asset = LocalDaemonAsset::new_from_path(entry?.path()).await?;
                let template = templates_map
                    .get(
                        &fs::read_to_string(local_daemon_asset.path.join("assets_sha3_256"))
                            .unwrap_or("default".to_string()),
                    )
                    .unwrap_or(templates_map.get(&"default".to_string()).unwrap());
                template.reconciliation(&local_daemon_asset).await?;

                host_assets.local_daemons.lock().await.insert(
                    local_daemon_asset.plugin_daemon.public_key.clone(),
                    local_daemon_asset,
                );
            }
        }

        let remote_daemon_dir = host_assets.path.join("daemons").join("remote");
        if remote_daemon_dir.exists() {
            for entry in remote_daemon_dir.read_dir()? {
                let remote_daemon_asset = RemoteDaemonAsset::new_from_path(entry?.path()).await?;

                let daemon_key = urlencoding::encode(&remote_daemon_asset.remote_daemon.address);
                host_assets
                    .remote_daemons
                    .lock()
                    .await
                    .insert(daemon_key.to_string(), remote_daemon_asset);
            }
        }

        Ok(host_assets)
    }

    pub async fn up(&self) -> anyhow::Result<()> {
        for daemon in self.local_daemons.lock().await.values() {
            daemon.up().await?;
        }

        Ok(())
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        for daemon in self.local_daemons.lock().await.values() {
            daemon.down().await?;
        }

        Ok(())
    }

    pub async fn append_local_daemon(&self, plugin_daemon: Daemon) -> anyhow::Result<()> {
        let daemon_dir = self
            .path
            .join("daemons")
            .join("local")
            .join(&plugin_daemon.public_key);

        if !daemon_dir.exists() {
            fs::create_dir_all(&daemon_dir)?;
        }

        fs::write(
            daemon_dir.join("daemon.json"),
            serde_json::to_string(&plugin_daemon)?,
        )?;

        let daemon_asset = LocalDaemonAsset::new_from_path(daemon_dir).await?;
        daemon_asset.up().await?;

        let templates_map = self.templates.lock().await;
        let template = templates_map
            .get(
                &fs::read_to_string(daemon_asset.path.join("assets_sha3_256"))
                    .unwrap_or("default".to_string()),
            )
            .unwrap_or(templates_map.get(&"default".to_string()).unwrap());
        template.reconciliation(&daemon_asset).await?;

        self.local_daemons
            .lock()
            .await
            .insert(plugin_daemon.public_key.clone(), daemon_asset);

        Ok(())
    }

    pub async fn delete_local_daemon(&self, public_key: String) -> anyhow::Result<()> {
        match self.local_daemons.lock().await.remove(&public_key) {
            None => (),
            Some(asset) => {
                asset.down().await?;
                fs::remove_dir_all(asset.path)?;
            }
        }

        Ok(())
    }

    pub async fn append_remote_daemon(&self, remote_daemon: RemoteDaemon) -> anyhow::Result<()> {
        let daemon_key = urlencoding::encode(&remote_daemon.address);
        let daemon_dir = self
            .path
            .join("daemons")
            .join("remote")
            .join(daemon_key.as_ref());
        if !daemon_dir.exists() {
            fs::create_dir_all(&daemon_dir)?;
        }

        let daemon_file_path = daemon_dir.join("daemon.json");
        fs::write(&daemon_file_path, serde_json::to_string(&remote_daemon)?)?;

        let remote_daemon_asset = RemoteDaemonAsset::new_from_path(daemon_dir).await?;
        let daemon_key = urlencoding::encode(&remote_daemon.address);
        self.remote_daemons
            .lock()
            .await
            .insert(daemon_key.to_string(), remote_daemon_asset);

        Ok(())
    }

    pub async fn delete_remote_daemon(&self, address: String) -> anyhow::Result<()> {
        let daemon_key = urlencoding::encode(&address);
        self.remote_daemons
            .lock()
            .await
            .remove(&daemon_key.to_string());

        let daemon_dir = self
            .path
            .join("daemons")
            .join("remote")
            .join(daemon_key.as_ref());
        if daemon_dir.exists() {
            fs::remove_dir_all(&daemon_dir)?;
        }

        Ok(())
    }
}
