use std::{collections::HashMap, ops::Deref, path::PathBuf, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use connection::Connection;
use handlers::{connect_handler, regist_handler};
use plugin::models::PluginConfig;
use serde_json::{json, Value};
use tokio::sync::{broadcast::Sender, Mutex};
use tower_http::services::{ServeDir, ServeFile};
use typings::{DaemonChannelType, SignRequest, VerifyRequest, VerifyResponse};

mod connection;
mod handlers;
mod typings;

use crate::daemon::{PluginDaemon, SignBox};

pub struct PluginDaemonService {
    pub plugin_daemon: PluginDaemon,
    pub registed_plugins: Arc<Mutex<HashMap<String, PluginConfig>>>,
    channel: Sender<DaemonChannelType>,
    connections: Mutex<HashMap<String, Arc<Connection>>>,
}

impl PluginDaemonService {
    pub async fn new(daemon: PluginDaemon, assets_path: PathBuf, port: u16) -> anyhow::Result<Arc<Self>> {
        let tcp_listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let address = format!("http://{}", tcp_listener.local_addr()?.to_string());

        let mut plugin_daemon = daemon.clone();
        plugin_daemon.address = Some(address);

        let (tx, _rx) = tokio::sync::broadcast::channel::<DaemonChannelType>(16);

        let service = PluginDaemonService {
            plugin_daemon,
            registed_plugins: Arc::new(Mutex::new(HashMap::new())),
            channel: tx,
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
                    .route("/api/sig", post(sign_handler))
                    .route("/api/verify", post(verify_handler))
                    .route("/api/connect", get(connect_handler))
                    .fallback_service(serve_dir)
                    .with_state(service.clone());
                axum::serve(tcp_listener, app)
                    .with_graceful_shutdown(async move {
                        loop {
                            match service.channel.subscribe().recv().await.unwrap() {
                                DaemonChannelType::Terminate => break,
                                _ => (),
                            }
                        }
                    })
                    .await
                    .unwrap();
            }
        });

        Ok(service)
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        self.channel.send(DaemonChannelType::Terminate)?;
        Ok(())
    }

    pub async fn wait(&self) -> anyhow::Result<()> {
        self.channel.subscribe().recv().await?;
        Ok(())
    }
}

async fn root_handler(
    State(service): State<Arc<PluginDaemonService>>,
) -> (StatusCode, Json<Value>) {
    let out = json!({
        "daemon": {
            "public_key": &service.plugin_daemon.public_key,
        },
        "plugins": service.registed_plugins.lock().await.deref(),
    });
    (StatusCode::OK, Json(out))
}

async fn sign_handler(
    State(state): State<Arc<PluginDaemonService>>,
    Json(payload): Json<SignRequest>,
) -> Result<Json<SignBox>, (StatusCode, String)> {
    let sign = state
        .plugin_daemon
        .sign(payload.base64_url_data_string.clone())
        .expect("create signature failed");

    Ok(Json(sign))
}

async fn verify_handler(
    State(_state): State<Arc<PluginDaemonService>>,
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
