use std::{
    collections::HashMap,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::utils;

pub struct PlatXDaemon {
    pub addr: String,
    pub plugin_map: Arc<Mutex<HashMap<String, RegistedPlugin>>>,
    stop_server_signal: Option<Sender<()>>,
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
            plugin_map: Arc::new(Mutex::new(HashMap::new())),
            stop_server_signal: None,
        }
    }

    pub async fn start_server(&mut self) -> anyhow::Result<JoinHandle<()>> {
        let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        self.addr = format!("http://{}", tcp_listener.local_addr().unwrap().to_string());

        let router = Router::new()
            .route("/plugin", post(regist_handler).get(get_plugins_handler))
            .with_state(self.plugin_map.clone());
        let (handler, tx) =
            utils::start_server_with_graceful_shutdown_channel(tcp_listener, router);
        self.stop_server_signal = Some(tx);

        Ok(handler)
    }

    pub fn serilize_plugins(&self) -> anyhow::Result<String> {
        let plugin_map = self.plugin_map.lock().unwrap();
        let json_string = serde_json::to_string(plugin_map.deref())?;
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
    pub entries: Vec<PlatXEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatXEntry {
    pub label: String,
    pub icon: String,
    pub href: String,
    pub target: String,
}

impl PlatXConfig {
    pub fn from_path(dir_path: PathBuf) -> anyhow::Result<Self> {
        let plugin_file = dir_path.join("plugin.json");
        let plugin_bytes = std::fs::read(plugin_file)?;
        let config: PlatXConfig = serde_json::from_slice(&plugin_bytes)?;

        Ok(config)
    }
}

async fn regist_handler(
    State(state): State<Arc<Mutex<HashMap<String, RegistedPlugin>>>>,
    Json(target): Json<Value>,
) -> String {
    let addr = target
        .as_object()
        .expect("invalid input")
        .get("addr")
        .expect("parse addr failed")
        .as_str()
        .expect("parse addr as string failed");
    let target = url::Url::parse(addr).expect("parse addr as url failed");

    let config = reqwest::get(target.join("plugin.json").unwrap())
        .await
        .expect("request regist plugin failed")
        .json::<PlatXConfig>()
        .await
        .expect("json deserilize failed");
    println!("plugin {} registed", config.name);

    let registed_plugin = RegistedPlugin {
        addr: addr.to_string(),
        config,
    };
    state
        .lock()
        .unwrap()
        .insert(registed_plugin.config.name.clone(), registed_plugin);

    "OK".to_string()
}

async fn get_plugins_handler(
    State(state): State<Arc<Mutex<HashMap<String, RegistedPlugin>>>>,
) -> String {
    serde_json::to_string(state.lock().unwrap().deref()).expect("json stringify plugin map failed")
}
