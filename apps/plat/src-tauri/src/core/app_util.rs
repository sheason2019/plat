use std::{collections::HashMap, sync::Arc};

use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc::Sender, Mutex};

#[derive(Debug, Clone)]
pub struct AppUtil {
    app_handle: AppHandle,
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
    ) -> bool {
        let channel = self
            .make_channel(
                "confirm-sign".to_string(),
                json!({
                    "plugin_name": "TODO",
                    "public_key": "TODO",
                    "data": base64_url_data_string,
                    "describe": describe,
                }),
            )
            .await;

        channel.get("allow").unwrap().as_bool().unwrap()
    }
}
