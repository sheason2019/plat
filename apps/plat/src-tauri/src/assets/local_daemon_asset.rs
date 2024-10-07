use std::{fs, ops::DerefMut, path::PathBuf, sync::Arc};

use anyhow::anyhow;
use daemon::{daemon::Daemon, service::DaemonServer};
use serde_json::{json, Value};
use tokio::sync::Mutex;

pub struct LocalDaemonAsset {
    pub path: PathBuf,
    pub plugin_daemon: Daemon,
    plugin_daemon_service: Mutex<Option<Arc<DaemonServer>>>,
}

impl LocalDaemonAsset {
    pub async fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        let plugin_daemon_bytes = fs::read(path.join("daemon.json"))?;
        let plugin_daemon: Daemon = serde_json::from_slice(&plugin_daemon_bytes)?;

        let daemon_asset = LocalDaemonAsset {
            path,
            plugin_daemon,
            plugin_daemon_service: Mutex::new(None),
        };

        Ok(daemon_asset)
    }

    pub async fn to_json_string(&self) -> anyhow::Result<Value> {
        let service = self.plugin_daemon_service.lock().await;
        let service = match service.as_ref() {
            Some(service) => service,
            None => return Err(anyhow!("Local Daemon 服务尚未启动")),
        };
        let value = json!({
            "public_key": &service.daemon.public_key,
            "password": &service.daemon.password,
            "address": &service.address,
        });
        Ok(value)
    }

    pub async fn up(&self) -> anyhow::Result<()> {
        let mut plugin_daemon_service_option = self.plugin_daemon_service.lock().await;
        if plugin_daemon_service_option.is_some() {
            return Ok(());
        }

        let plugin_daemon_service =
            DaemonServer::new(self.plugin_daemon.clone(), self.path.clone(), 0).await?;
        plugin_daemon_service_option.replace(plugin_daemon_service);

        Ok(())
    }

    pub async fn down(&self) -> anyhow::Result<()> {
        let mut lock = self.plugin_daemon_service.lock().await;
        if lock.is_some() {
            lock.as_ref().unwrap().stop().await?;
        }

        *lock.deref_mut() = None;

        Ok(())
    }

    pub async fn update_password(&mut self, new_password: String) -> anyhow::Result<()> {
        // 停止服务
        self.down().await?;

        // 修改 daemon
        self.plugin_daemon.password = new_password;
        fs::write(
            self.path.join("daemon.json"),
            serde_json::to_string(&self.plugin_daemon)?,
        )?;

        // 重启服务
        self.up().await?;

        Ok(())
    }
}
