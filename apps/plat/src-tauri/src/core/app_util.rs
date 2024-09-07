use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use serde_json::json;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{mpsc::Sender, Mutex};

#[derive(Debug, Clone)]
pub struct AppUtil {
    pub app_handle: AppHandle,
    channel_map: Arc<Mutex<HashMap<String, Sender<serde_json::Value>>>>,
}

impl AppUtil {
    pub fn new(app_handle: AppHandle) -> Self {
        AppUtil {
            app_handle,
            channel_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn make_channel(&self, ty: String, data: serde_json::Value) -> serde_json::Value {
        let id = uuid::Uuid::new_v4().to_string();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<serde_json::Value>(1);

        let channel_map = self.channel_map.clone();
        channel_map.lock().await.insert(id.clone(), tx);

        println!("send channel data");
        self.app_handle
            .emit(
                "channel",
                json!({
                    "id": id,
                    "type": ty,
                    "data": data,
                }),
            )
            .unwrap();

        tokio::task::spawn(async move {
            let val = rx.recv().await.unwrap();
            channel_map.lock().await.remove(&id);
            val
        })
        .await
        .unwrap()
    }

    pub async fn complete_channel(&self, id: String, data: serde_json::Value) {
        let channel_map = self.channel_map.lock().await;
        let channel = match channel_map.get(&id) {
            Some(value) => value,
            None => return,
        };
        let channel = channel.clone();
        drop(channel_map);

        channel.clone().send(data).await.unwrap();
    }
}

impl AppUtil {
    pub async fn confirm_sign_dialog(
        &self,
        base64_url_data_string: String,
        describe: String,
        public_key: String,
    ) -> bool {
        let channel = self
            .make_channel(
                "confirm-sign".to_string(),
                json!({
                    "plugin_name": "TODO",
                    "public_key": public_key,
                    "data": base64_url_data_string,
                    "describe": describe,
                }),
            )
            .await;

        channel.get("allow").unwrap().as_bool().unwrap()
    }

    pub async fn confirm_install_plugin_dialog(
        &self,
        public_key: String,
        plugin_file_path: PathBuf,
    ) -> anyhow::Result<Option<PathBuf>> {
        // 创建空的 Plugin 缓存文件夹
        let plugin_root_directory = self
            .app_handle
            .path()
            .app_data_dir()?
            .join(public_key.clone())
            .join("plugins");
        if !plugin_root_directory.exists() {
            fs::create_dir_all(&plugin_root_directory)?;
        }

        let plugin_cache_directory = plugin_root_directory.join(".cache");
        if plugin_cache_directory.exists() {
            fs::remove_dir_all(&plugin_cache_directory)?;
        }
        fs::create_dir(&plugin_cache_directory)?;

        // 1. 首先将插件内容解压到缓存文件夹
        let untarer = bundler::untarer::Untarer::new(plugin_file_path);
        untarer.untar(plugin_cache_directory.clone())?;

        let plugin_file = plugin_cache_directory.join("plugin.json");
        let plugin_bytes = fs::read(plugin_file)?;
        let plugin: models::PluginConfig = serde_json::from_slice(&plugin_bytes)?;
        // 2. 发出 Channel 等待用户确认安装 Plugin
        let channel = self
            .make_channel(
                "confirm-install-plugin".to_string(),
                json!({
                    "public_key": public_key,
                    "plugin": &plugin,
                }),
            )
            .await;
        let allow = channel.get("allow").unwrap().as_bool().unwrap();

        // 用户拒绝安装则删除缓存文件夹
        if !allow {
            fs::remove_dir_all(plugin_cache_directory)?;
            return Ok(None);
        }

        // 用户同意则将插件移动到插件文件夹
        let plugin_directory =
            plugin_root_directory.join(urlencoding::encode(&plugin.name).to_string());
        if !plugin_directory.exists() {
            fs::create_dir_all(plugin_directory.clone())?;
        }
        fs::remove_dir_all(plugin_directory.clone())?;
        fs::rename(plugin_cache_directory, &plugin_directory)?;

        Ok(Some(plugin_directory))
    }
}
