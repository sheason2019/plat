use std::{collections::HashMap, net::ToSocketAddrs, path::PathBuf};

use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::utils;

pub struct PlatXDaemon {
    pub addr: String,
    stop_server_signal: Option<Sender<()>>,
    plugins: HashMap<String, RegistedPlugin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistedPlugin {
    pub addr: String,
    pub config: PlatXConfig,
}

impl PlatXDaemon {
    pub fn new() -> Self {
        PlatXDaemon {
            addr: String::new(),
            plugins: HashMap::new(),
            stop_server_signal: None,
        }
    }

    pub async fn start_server(&mut self) -> anyhow::Result<JoinHandle<()>> {
        let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        self.addr = format!("http://{}", tcp_listener.local_addr().unwrap().to_string());

        let router = Router::new().route("/plugin", post(regist_handler));
        let (handler, tx) =
            utils::start_server_with_graceful_shutdown_channel(tcp_listener, router);
        self.stop_server_signal = Some(tx);

        Ok(handler)
    }

    pub fn serilize_plugins(&self) -> anyhow::Result<String> {
        let json_string = serde_json::to_string(&self.plugins)?;
        Ok(json_string)
    }

    pub fn uninstall_plugin(&mut self, name: String) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatXConfig {
    pub name: String,
    pub wasm_root: String,
    pub assets_root: String,
}

impl PlatXConfig {
    pub fn from_path(dir_path: PathBuf) -> anyhow::Result<Self> {
        let plugin_file = dir_path.join("plugin.json");
        let plugin_bytes = std::fs::read(plugin_file)?;
        let config: PlatXConfig = serde_json::from_slice(&plugin_bytes)?;

        Ok(config)
    }
}

async fn regist_handler(Json(target): Json<String>) -> String {
    println!("regist plugin {}", target);
    "regist plugin".to_string()
}
