use std::{collections::HashMap, fs, sync::Arc};

use axum::{
    extract::{ws::Message, Query, State},
    Json,
};
use plugin::models::Plugin;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::broadcast::Sender;

use crate::service::{
    typings::{AppError, ConnectionMessage},
    DaemonServer,
};

pub async fn delete_plugin_handler(
    State(server): State<Arc<DaemonServer>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    let name = query.get("name").unwrap().clone();
    try_delete_plugin_handler(server, name).await?;
    Ok(Json(json!({"complete": true})))
}

async fn try_delete_plugin_handler(
    server: Arc<DaemonServer>,
    name: String,
) -> anyhow::Result<bool> {
    // 从 Plugins 中读取需要删除的插件信息并发送给用户
    let plugin = { server.plugins.lock().await.get(&name).unwrap().clone() };
    {
        let event = ConnectionMessage {
            ty: String::from("confirm/delete-plugin"),
            payload: serde_json::to_value(DeletePluginRequest {
                plugin: plugin.clone(),
            })?,
        };
        let message = Message::Text(serde_json::to_string(&event)?);
        for connection in server.connections.lock().await.iter() {
            connection.send_message(message.clone())?;
        }
    }

    let allow_chan: Sender<bool> = Sender::new(4);
    let mut allow_result = allow_chan.subscribe();
    for connection in server.connections.lock().await.iter() {
        let connection = connection.clone();
        let allow_chan = allow_chan.clone();
        let name = name.clone();
        tokio::spawn(async move {
            let mut receive_sub = connection.receive_channel.subscribe();
            let mut allow_sub = allow_chan.subscribe();
            loop {
                tokio::select! {
                    _ = allow_sub.recv() => break,
                    message = receive_sub.recv() => {
                        let message = match message {
                            Ok(value) => value,
                            Err(_) => break,
                        };
                        let content = match message {
                            Message::Text(content) => content,
                            _ => continue,
                        };
                        let connection_message: ConnectionMessage = match serde_json::from_str(&content) {
                            Ok(value) => value,
                            Err(_) => continue,
                        };
                        if connection_message.ty != "confirm/delete-plugin" {
                            continue;
                        }
                        let delete_plugin_response: DeletePluginResponse = match serde_json::from_value(connection_message.payload) {
                            Ok(value) => value,
                            Err(_) => continue,
                        };
                        if delete_plugin_response.name != name {
                            continue;
                        }
                        let _ = allow_chan.send(delete_plugin_response.allow);
                    },
                }
            }
        });
    }

    if !allow_result.recv().await? {
        return Ok(false);
    }

    let _ = server.plugins.lock().await.remove(&name);
    match server.plugin_servers.lock().await.remove(&name) {
        None => (),
        Some(plugin_server) => {
            plugin_server.stop().await;
            fs::remove_dir_all(plugin_server.path)?;
        }
    }

    for connection in server.connections.lock().await.iter() {
        connection.send_daemon(&server).await?;
    }

    Ok(true)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeletePluginRequest {
    plugin: Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeletePluginResponse {
    name: String,
    allow: bool,
}
