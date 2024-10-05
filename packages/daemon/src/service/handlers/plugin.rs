use std::{
    fs::{self},
    sync::Arc,
};

use axum::{
    extract::{Multipart, State},
    Json,
};
use futures::TryStreamExt;
use plugin::models::PluginConfig;
use reqwest::StatusCode;
use serde_json::{json, Value};
use tokio::{
    fs::File,
    io::{self, BufWriter},
};
use tokio_util::io::StreamReader;

use crate::service::DaemonServer;

pub async fn list_plugin_handler(
    State(service): State<Arc<DaemonServer>>,
) -> (StatusCode, Json<Value>) {
    let registed_plugins = service.registed_plugins.lock().await;
    let plugins: Vec<&PluginConfig> = registed_plugins.values().collect();
    let plugins = json!({
        "plugins": &plugins,
    });

    (StatusCode::OK, Json(plugins))
}

pub async fn install_plugin_handler(
    State(service): State<Arc<DaemonServer>>,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, String)> {
    // 获取上传的文件
    let field = match multipart.next_field().await {
        Ok(Some(val)) => val,
        _ => return Err((StatusCode::BAD_REQUEST, String::from("无法找到Plugin文件"))),
    };

    let file_name = match field.file_name() {
        Some(name) => name,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("无法找到Plugin文件名称"),
            ))
        }
    };

    // 将用户上传的插件复制至 cache 文件夹，并解压到对应目录
    let cache_dir = service.root_path.join(".cache").join(file_name);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).unwrap();
    }
    let tar_file_path = cache_dir.join("plugin.tar.gz");
    let mut tar_file = BufWriter::new(File::create(&tar_file_path).await.unwrap());
    let mut reader =
        StreamReader::new(field.map_err(|err| io::Error::new(io::ErrorKind::Other, err)));
    io::copy(&mut reader, &mut tar_file).await.unwrap();
    let out_dir = cache_dir.join("out");
    bundler::plugin::untar(tar_file_path, out_dir).unwrap();

    // 读取插件信息
    println!("读取插件信息");

    // 获取 Connection，请求用户确认
    println!("尝试通过 Connection 确认用户操作");

    // 根据返回的结果安装或取消安装插件

    // 尝试刷新 assets

    todo!()
}

pub async fn delete_plugin_handler(State(service): State<Arc<DaemonServer>>) {
    // 从 Plugins 中读取需要删除的插件
    // 获取 Connection，请求用户确认
    // 根据返回的结果删除或取消删除插件
    // 尝试刷新 assets
}
