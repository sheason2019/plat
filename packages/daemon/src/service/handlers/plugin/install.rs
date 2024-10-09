use std::{
    fs::{self},
    sync::Arc,
};

use anyhow::{bail, Context};
use axum::{
    extract::{ws::Message, Multipart, State},
    Json,
};
use futures::TryStreamExt;
use plugin::{models::Plugin, Options, PluginServer};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::{
    fs::File,
    io::{self, BufWriter},
    sync::broadcast::Sender,
};
use tokio_util::io::StreamReader;

use crate::service::{
    typings::{AppError, ConnectionMessage},
    DaemonServer,
};

pub async fn install_plugin_handler(
    State(server): State<Arc<DaemonServer>>,
    multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    let response = try_install_plugin(server, multipart).await?;
    Ok(response)
}

async fn try_install_plugin(
    server: Arc<DaemonServer>,
    mut multipart: Multipart,
) -> Result<Json<Value>, anyhow::Error> {
    // 获取上传的文件
    let field = match multipart.next_field().await? {
        Some(value) => value,
        None => bail!("读取插件安装包失败"),
    };

    let file_name = match field.file_name() {
        Some(name) => name.to_string(),
        None => bail!("读取Plugin文件名称失败"),
    };

    // 将用户上传的插件复制至 cache 文件夹，并解压到对应目录
    let cache_dir = server.root_path.join(".cache").join(&file_name);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }

    let tar_file_path = cache_dir.join("plugin.tar.gz");
    let mut tar_file = BufWriter::new(File::create(&tar_file_path).await?);
    let mut reader =
        StreamReader::new(field.map_err(|err| io::Error::new(io::ErrorKind::Other, err)));
    io::copy(&mut reader, &mut tar_file).await?;
    let out_dir = cache_dir.join("out");
    bundler::plugin::untar(tar_file_path, out_dir.clone())?;

    // 读取插件信息
    let plugin: Plugin = serde_json::from_slice(&fs::read(out_dir.join("plugin.json"))?)?;
    let connection_message = ConnectionMessage {
        ty: String::from("confirm/install-plugin"),
        payload: serde_json::to_value(InstallPluginRequest {
            name: file_name.clone(),
            plugin: plugin.clone(),
        })?,
    };

    // 获取 Connection，请求用户确认
    let message = Message::Text(serde_json::to_string(&connection_message)?);
    for connection in server.connections.lock().await.iter() {
        connection.send_message(message.clone())?;
    }
    let allow_chan: Sender<bool> = Sender::new(4);
    let mut allow_result = allow_chan.subscribe();
    for connection in server.connections.lock().await.iter() {
        let connection = connection.clone();
        let allow_chan = allow_chan.clone();
        let name = file_name.clone();
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
                        if connection_message.ty != "confirm/install-plugin" {
                            continue;
                        }
                        let install_plugin_response: InstallPluginResponse = match serde_json::from_value(connection_message.payload) {
                            Ok(value) => value,
                            Err(_) => continue,
                        };
                        if install_plugin_response.name != name {
                            continue;
                        }
                        let _ = allow_chan.send(install_plugin_response.allow);
                    },
                }
            }
        });
    }

    // 根据返回的结果安装或取消安装插件
    if !allow_result.recv().await? {
        fs::remove_dir_all(&cache_dir)?;
        return Ok(Json(json!({"complete": false})));
    }

    let plugin_dir = server
        .root_path
        .join("plugins")
        .join(&urlencoding::encode(&plugin.name).to_string());
    let plugins_dir = plugin_dir.parent().unwrap();
    if !plugins_dir.exists() {
        fs::create_dir_all(plugins_dir)?;
    }
    if plugin_dir.exists() {
        let storage_dir = plugin_dir.join("storage");
        if storage_dir.exists() {
            fs::rename(storage_dir, out_dir.join("storage")).context("移动 Storage 目录失败")?
        }
        fs::remove_dir_all(&plugin_dir)?;
    }
    fs::rename(&out_dir, &plugin_dir).context("移动插件至插件目录失败")?;
    fs::remove_dir_all(&cache_dir)?;

    // 启动插件
    let plugin_server = PluginServer::new(
        plugin_dir,
        Options {
            port: 0,
            daemon_address: server.address.clone(),
            regist_address: None,
        },
    )
    .await
    .context("启动插件失败")?;
    server
        .plugin_servers
        .lock()
        .await
        .insert(plugin_server.plugin().name.clone(), plugin_server);
    for connection in server.connections.lock().await.iter() {
        connection.send_daemon(&server).await?;
    }

    Ok(Json(json!({"complete": true})))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallPluginRequest {
    name: String,
    plugin: Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallPluginResponse {
    name: String,
    allow: bool,
}
