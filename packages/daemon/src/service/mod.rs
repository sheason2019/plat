use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use connection::Connection;
use handlers::{
    connect_handler, delete_plugin_handler, install_plugin_handler, list_plugin_handler,
    regist_handler, sig_handler,
};
use plugin::{models::PluginConfig, Options, PluginServer};
use serde_json::{json, Value};
use tokio::sync::{broadcast::Sender, Mutex};
use tower_http::services::{ServeDir, ServeFile};
use typings::{VerifyRequest, VerifyResponse};

mod connection;
mod handlers;
mod typings;

use crate::daemon::{PluginDaemon, SignBox};

pub struct DaemonServer {
    pub plugin_daemon: PluginDaemon,
    pub registed_plugins: Arc<Mutex<HashMap<String, PluginConfig>>>,
    pub address: String,
    root_path: PathBuf,
    terminate: Sender<()>,
    connections: Mutex<HashMap<String, Arc<Connection>>>,
}

impl DaemonServer {
    pub async fn new(
        daemon: PluginDaemon,
        root_path: PathBuf,
        port: u16,
    ) -> anyhow::Result<Arc<Self>> {
        let assets_path = root_path.join("assets");
        let tcp_listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let address = format!("http://{}", tcp_listener.local_addr()?.to_string());

        let (tx, _rx) = tokio::sync::broadcast::channel::<()>(4);

        let service = DaemonServer {
            plugin_daemon: daemon,
            registed_plugins: Arc::new(Mutex::new(HashMap::new())),
            address,
            root_path,
            terminate: tx,
            connections: Mutex::new(HashMap::new()),
        };
        let service = Arc::new(service);

        tokio::task::spawn({
            let service = service.clone();
            async move {
                let serve_dir = ServeDir::new(assets_path.clone())
                    .not_found_service(ServeFile::new(assets_path.join("index.html")));

                let app = Router::new()
                    .route("/api", get(root_handler))
                    .route("/api/regist", get(regist_handler))
                    .route("/api/sig", post(sig_handler))
                    .route("/api/verify", post(verify_handler))
                    .route("/api/connect", get(connect_handler))
                    .route(
                        "/api/plugin",
                        get(list_plugin_handler)
                            .post(install_plugin_handler)
                            .delete(delete_plugin_handler),
                    )
                    .fallback_service(serve_dir)
                    .with_state(service.clone());
                axum::serve(tcp_listener, app)
                    .with_graceful_shutdown(async move {
                        let _ = service.terminate.subscribe().recv().await;
                    })
                    .await
                    .unwrap();
            }
        });

        Ok(service)
    }

    pub async fn start_local_plugin(&self) -> anyhow::Result<()> {
        let plugins_dir = self.root_path.join("plugins");
        if !plugins_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&plugins_dir)? {
            let entry = entry?;
            let plugin_server = PluginServer::new(
                entry.path(),
                Options {
                    port: 0,
                    daemon_address: String::new(),
                    regist_address: None,
                },
            )
            .await?;
        }
        todo!()
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        self.terminate.send(())?;
        Ok(())
    }

    pub async fn wait(&self) -> anyhow::Result<()> {
        self.terminate.subscribe().recv().await?;
        Ok(())
    }
}

async fn root_handler(State(service): State<Arc<DaemonServer>>) -> (StatusCode, Json<Value>) {
    let out = json!({
        "public_key": &service.plugin_daemon.public_key,
    });
    (StatusCode::OK, Json(out))
}

async fn verify_handler(
    State(_state): State<Arc<DaemonServer>>,
    Json(payload): Json<VerifyRequest>,
) -> (StatusCode, Json<VerifyResponse>) {
    let sign_box = SignBox {
        public_key: payload.public_key,
        signature: payload.signature,
    };
    let result = sign_box.verify(payload.base64_url_data_string);

    (
        StatusCode::OK,
        Json(VerifyResponse {
            success: result.is_ok(),
        }),
    )
}
