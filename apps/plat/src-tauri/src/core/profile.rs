use std::{
    fs::{self},
    ops::Deref,
    path::{self, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use daemon::{daemon::PluginDaemon, service::PluginDaemonService};
use serde_json::{json, Value};

pub struct Profile {
    pub data_root: PathBuf,
    pub daemon_services: Vec<Arc<PluginDaemonService>>,
}

impl Profile {
    pub async fn init(data_root: PathBuf) -> anyhow::Result<Self> {
        let mut daemon_services: Vec<Arc<PluginDaemonService>> = Vec::new();
        let data_root = path::Path::new(&data_root);
        if !data_root.exists() {
            fs::create_dir_all(data_root).context("create data_root failed")?;
        }

        let read_dir = std::fs::read_dir(data_root).context("read dir failed")?;
        for dir in read_dir {
            let dir = dir?;

            let filename = dir.file_name().into_string().unwrap();
            if filename.starts_with(".") {
                continue;
            }

            let daemon_file = dir.path().join("daemon.json");
            if !daemon_file.exists() {
                continue;
            }

            let daemon = PluginDaemon::from_directory(dir.path())?;
            let service = PluginDaemonService::new(daemon, 0).await?;

            daemon_services.push(service);
        }

        Ok(Profile {
            data_root: data_root.to_path_buf(),
            daemon_services,
        })
    }

    pub fn to_json_string(&self) -> String {
        let mut daemons: Vec<Value> = Vec::new();
        for daemon in &self.daemon_services {
            let plugin_map = daemon.registed_plugins.lock().unwrap();
            let daemon_json = json!({
                "public_key": daemon.plugin_daemon.public_key,
                "daemon_address": daemon.addr,
                "registed_plugins": plugin_map.deref(),
            });
            daemons.push(daemon_json);
        }

        let value = json!({
            "daemons": &daemons,
        });

        serde_json::to_string(&value).unwrap()
    }

    pub async fn generate_isolate(&mut self) -> anyhow::Result<String> {
        let daemon = PluginDaemon::generate(self.data_root.clone())?;
        let public_key = daemon.public_key.clone();

        let service = PluginDaemonService::new(daemon, 0).await?;
        self.daemon_services.push(service);

        Ok(public_key)
    }

    pub async fn delete_isolate(&mut self, public_key: String) -> anyhow::Result<()> {
        // 在内存中删除 isolate
        let position = self
            .daemon_services
            .iter()
            .position(|item| item.plugin_daemon.public_key == public_key)
            .expect("cannot find position");
        let item = self.daemon_services.remove(position);
        item.stop().await?;

        // 在文件系统中删除 isolate
        let p = self.data_root.join(public_key);
        fs::remove_dir_all(p)?;

        Ok(())
    }
}
