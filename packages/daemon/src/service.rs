use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use models::{PluginConfig, RegistedPlugin};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

use crate::daemon::PluginDaemon;

pub struct PluginDaemonService {
    pub plugin_daemon: PluginDaemon,
    pub addr: String,
    pub registed_plugins: Arc<Mutex<HashMap<String, models::RegistedPlugin>>>,

    service_stop_sender: Sender<()>,
    confirm_signature_handler: Option<fn() -> bool>,
}

impl PluginDaemonService {
    pub async fn new(daemon: PluginDaemon, port: u16) -> anyhow::Result<Arc<Self>> {
        let tcp_listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let addr = format!("http://{}", tcp_listener.local_addr()?.to_string());

        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

        let service = PluginDaemonService {
            plugin_daemon: daemon.clone(),
            addr,
            registed_plugins: Arc::new(Mutex::new(HashMap::new())),

            service_stop_sender: tx.clone(),
            confirm_signature_handler: None,
        };
        let service = Arc::new(service);

        tokio::task::spawn({
            let service = service.clone();
            async move {
                let app = Router::new()
                    .route("/", get(root_handler))
                    .route("/plugin", post(regist_plugin_handler))
                    .with_state((
                        service.plugin_daemon.clone(),
                        service.registed_plugins.clone(),
                    ));
                axum::serve(tcp_listener, app)
                    .with_graceful_shutdown(async move {
                        rx.recv().await;
                    })
                    .await
                    .unwrap();
            }
        });

        Ok(service)
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        self.service_stop_sender.send(()).await?;
        Ok(())
    }
}

async fn root_handler(
    State((daemon, registed_plugins)): State<(
        PluginDaemon,
        Arc<Mutex<HashMap<String, models::RegistedPlugin>>>,
    )>,
) -> (StatusCode, Json<Value>) {
    let plugins = registed_plugins.lock().unwrap().clone();

    let out = json!({
        "daemon": {
            "public_key": &daemon.public_key,
        },
        "plugins": plugins,
    });
    (StatusCode::OK, Json(out))
}

async fn regist_plugin_handler(
    State((_daemon, registed_plugins)): State<(
        PluginDaemon,
        Arc<Mutex<HashMap<String, models::RegistedPlugin>>>,
    )>,
    Json(payload): Json<Value>,
) -> &'static str {
    let addr = payload
        .as_object()
        .expect("invalid input")
        .get("addr")
        .expect("parse addr faield")
        .as_str()
        .expect("parse addr as string failed");
    let target =
        url::Url::parse(addr).expect(format!("parse addr {} as url failed", &addr).as_ref());

    let config = reqwest::get(target.join("plugin.json").unwrap())
        .await
        .expect("request regist plugin failed")
        .json::<PluginConfig>()
        .await
        .expect("json deserilize failed");
    println!("plugin {} registed", config.name);

    let registed_plugin = RegistedPlugin {
        addr: addr.to_string(),
        config,
    };

    registed_plugins
        .lock()
        .unwrap()
        .insert(registed_plugin.config.name.clone(), registed_plugin);

    "OK"
}
