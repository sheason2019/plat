use std::{
    collections::HashMap,
    fs::{self},
    ops::Deref,
    path::{self, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use daemon::{daemon::PluginDaemon, service::PluginDaemonService};
use models::PluginConfig;
use plugin::PluginService;
use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

use crate::core::app_util::AppUtil;

pub struct Profile {
    pub data_root: PathBuf,
    pub app_util: Arc<AppUtil>,
    pub daemon_service_map: HashMap<String, Arc<PluginDaemonService>>,
    plugin_service_map: HashMap<String, PluginService>,
}

impl Profile {
    pub async fn init(data_root: PathBuf, app_handle: AppHandle) -> anyhow::Result<Self> {
        let mut profile = Profile {
            data_root: data_root.to_path_buf(),
            daemon_service_map: HashMap::new(),
            plugin_service_map: HashMap::new(),
            app_util: Arc::new(AppUtil::new(app_handle)),
        };

        let data_root = path::Path::new(&data_root);
        if !data_root.exists() {
            fs::create_dir_all(data_root).context("create data_root failed")?;
        }

        let data_root_content = std::fs::read_dir(data_root).context("read dir failed")?;
        for daemon_directory in data_root_content {
            let daemon_directory = daemon_directory?;

            let filename = daemon_directory.file_name().into_string().unwrap();
            if filename.starts_with(".") {
                continue;
            }

            let daemon_file = daemon_directory.path().join("daemon.json");
            if !daemon_file.exists() {
                continue;
            }

            let app_util = profile.app_util.clone();
            let daemon = PluginDaemon::from_directory(daemon_directory.path())?;
            let public_key = daemon.public_key.clone();
            let daemon_service = PluginDaemonService::new(daemon, 0, move |req| {
                let app_util = app_util.clone();
                let public_key = public_key.clone();
                Box::pin(async move {
                    let allow = app_util
                        .confirm_sign_dialog(
                            req.base64_url_data_string,
                            "describe".to_string(),
                            public_key,
                        )
                        .await;
                    Ok(allow)
                })
            })
            .await?;

            match profile.daemon_service_map.insert(
                daemon_service.plugin_daemon.public_key.clone(),
                daemon_service.clone(),
            ) {
                None => (),
                Some(value) => {
                    value.stop().await?;
                }
            }

            let plugins_directory = daemon_directory.path().join("plugins");
            if !plugins_directory.exists() {
                fs::create_dir_all(&plugins_directory)?;
            }

            for plugins_directory_content in fs::read_dir(plugins_directory)? {
                let plugins_directory_content = plugins_directory_content?;
                if plugins_directory_content
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .starts_with(".")
                {
                    continue;
                }

                profile
                    .try_start_plugin_from_dir(
                        &daemon_service.plugin_daemon.public_key,
                        plugins_directory_content.path(),
                    )
                    .await?;
            }
        }

        Ok(profile)
    }

    pub async fn to_json_string(&self) -> String {
        let mut daemons: Vec<Value> = Vec::new();
        for (public_key, daemon) in &self.daemon_service_map {
            let plugin_map = daemon.registed_plugins.lock().await;
            let daemon_json = json!({
                "public_key": public_key,
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

    pub async fn generate_daemon_service(&mut self) -> anyhow::Result<String> {
        let daemon = PluginDaemon::generate(self.data_root.clone())?;
        let public_key = daemon.public_key.clone();
        let result = Ok(public_key.clone());

        let app_util = self.app_util.clone();
        let service = PluginDaemonService::new(daemon, 0, move |req| {
            let app_util = app_util.clone();
            let public_key = public_key.clone();
            Box::pin(async move {
                let allow = app_util
                    .confirm_sign_dialog(
                        req.base64_url_data_string,
                        "describe".to_string(),
                        public_key,
                    )
                    .await;
                Ok(allow)
            })
        })
        .await?;
        self.daemon_service_map
            .insert(service.plugin_daemon.public_key.clone(), service)
            .unwrap();

        result
    }

    pub async fn delete_daemon_service(&mut self, public_key: String) -> anyhow::Result<()> {
        // 在内存中删除 isolate
        let item = self.daemon_service_map.get(&public_key).unwrap();
        item.stop().await?;

        // 在文件系统中删除 isolate
        let p = self.data_root.join(public_key);
        fs::remove_dir_all(p)?;

        Ok(())
    }

    pub async fn try_start_plugin_from_dir(
        &mut self,
        public_key: &String,
        plugin_directory: PathBuf,
    ) -> anyhow::Result<()> {
        let daemon_service = self.daemon_service_map.get(public_key).unwrap();
        let plugin_config = PluginConfig::from_file(plugin_directory.join("plugin.json"))?;
        let service_key = format!(
            "{}.{}",
            &daemon_service.plugin_daemon.public_key, &plugin_config.name
        );

        match self.plugin_service_map.get(&service_key) {
            Some(value) => {
                value.stop().await;
                daemon_service.health_check().await;
            }
            None => (),
        }

        let service =
            plugin::PluginService::new(plugin_directory, daemon_service.addr.clone(), None, 0)
                .await?;
        self.plugin_service_map.insert(service_key, service);

        Ok(())
    }

    pub async fn remove_plugin(
        &mut self,
        public_key: &String,
        plugin_name: String,
    ) -> anyhow::Result<()> {
        match self
            .plugin_service_map
            .remove(&format!("{}.{}", public_key, &plugin_name))
        {
            Some(service) => {
                service.stop().await;
            }
            None => (),
        }

        self.daemon_service_map
            .get(public_key)
            .unwrap()
            .registed_plugins
            .lock()
            .await
            .remove(&plugin_name);

        let plugin_root_directory = self
            .app_util
            .app_handle
            .path()
            .app_data_dir()?
            .join(public_key.clone())
            .join("plugins")
            .join(urlencoding::encode(&plugin_name).to_string());
        if plugin_root_directory.exists() {
            fs::remove_dir_all(plugin_root_directory)?;
        }

        Ok(())
    }
}
