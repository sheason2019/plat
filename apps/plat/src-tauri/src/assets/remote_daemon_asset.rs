use std::{fs, path::PathBuf};

use serde_json::{json, Value};

use crate::typings::RemoteDaemon;

pub struct RemoteDaemonAsset {
    pub path: PathBuf,
    pub remote_daemon: RemoteDaemon,
}

impl RemoteDaemonAsset {
    pub async fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        let plugin_daemon_bytes = fs::read(path.join("daemon.json"))?;
        let remote_daemon: RemoteDaemon = serde_json::from_slice(&plugin_daemon_bytes)?;

        let daemon_asset = RemoteDaemonAsset {
            path,
            remote_daemon,
        };

        Ok(daemon_asset)
    }

    pub async fn to_json_string(&self) -> anyhow::Result<Value> {
        let value = json!({
            "address": &self.remote_daemon.address,
            "password": &self.remote_daemon.password,
        });
        Ok(value)
    }

    pub async fn update_password(&mut self, new_password: String) -> anyhow::Result<()> {
        self.remote_daemon.password = new_password;
        fs::write(
            self.path.join("daemon.json"),
            serde_json::to_string(&self.remote_daemon)?,
        )?;

        Ok(())
    }
}
