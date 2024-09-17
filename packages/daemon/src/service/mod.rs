use std::{borrow::Cow, collections::HashMap, ops::Deref, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{CloseFrame, Message},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures::{SinkExt, StreamExt};
use plugin::models::PluginConfig;
use serde_json::{json, Value};
use tokio::sync::{broadcast::Sender, Mutex};
use tower_http::services::{ServeDir, ServeFile};
use typings::{DaemonChannelType, SignRequest, VerifyRequest, VerifyResponse};

mod typings;

use crate::daemon::{PluginDaemon, SignBox};

pub struct PluginDaemonService {
    pub plugin_daemon: PluginDaemon,
    pub registed_plugins: Arc<Mutex<HashMap<String, PluginConfig>>>,

    channel: Sender<DaemonChannelType>,
}

impl PluginDaemonService {
    pub async fn new(daemon: PluginDaemon, port: u16) -> anyhow::Result<Arc<Self>> {
        let tcp_listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let address = format!("http://{}", tcp_listener.local_addr()?.to_string());

        let mut plugin_daemon = daemon.clone();
        plugin_daemon.address = Some(address);

        let (tx, _rx) = tokio::sync::broadcast::channel::<DaemonChannelType>(16);

        let service = PluginDaemonService {
            plugin_daemon,
            registed_plugins: Arc::new(Mutex::new(HashMap::new())),
            channel: tx,
        };
        let service = Arc::new(service);

        tokio::task::spawn({
            let service = service.clone();
            async move {
                let serve_dir =
                    ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

                let app = Router::new()
                    .route("/api", get(root_handler))
                    .route("/api/regist", get(regist_plugin_handler))
                    .route("/api/sig", post(sign_handler))
                    .route("/api/verify", post(verify_handler))
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

async fn regist_plugin_handler(
    State(service): State<Arc<PluginDaemonService>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        let (mut write, mut read) = socket.split();

        write
            .send(Message::Text("Ready".to_string()))
            .await
            .unwrap();

        let data = match read.next().await.unwrap().unwrap() {
            Message::Text(data) => data,
            _ => panic!("Message failed"),
        };
        let plugin_config: PluginConfig = serde_json::from_str(&data).unwrap();
        let name = plugin_config.name.clone();

        if service.registed_plugins.lock().await.contains_key(&name) {
            write
                .send(Message::Close(Some(CloseFrame {
                    code: 1000,
                    reason: Cow::from("已存在相同名称的 Plugin"),
                })))
                .await
                .unwrap();
            return;
        }

        service
            .registed_plugins
            .lock()
            .await
            .insert(name.clone(), plugin_config);

        let (tx, _rx) = tokio::sync::broadcast::channel::<()>(4);

        let recv_handler = tokio::task::spawn({
            let tx = tx.clone();
            async move {
                let mut sub = tx.subscribe();
                loop {
                    tokio::select! {
                        message_option = read.next() => {
                            match message_option {
                                None => break,
                                Some(message_result) => {
                                    match message_result {
                                        Err(_e) => break,
                                        Ok(message) => {
                                            match message {
                                                Message::Close(_) => break,
                                                _ => (),
                                            }
                                        },
                                    }
                                },
                            };
                        },
                        _ = tokio::time::sleep(Duration::from_secs(10)) => break,
                        _ = sub.recv() => break,
                    }
                }

                let _ = tx.send(());
            }
        });
        let ping_handler = tokio::task::spawn({
            let tx = tx.clone();
            async move {
                let mut sub = tx.subscribe();
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(4)) => {
                            write.send(Message::Ping(Vec::new())).await.unwrap();
                        },
                        _ = sub.recv() => break,
                    }
                }

                let _ = tx.send(());
            }
        });

        let _ = recv_handler.await;
        let _ = ping_handler.await;

        service.registed_plugins.lock().await.remove(&name);
    })
}

async fn sign_handler(
    State(state): State<Arc<PluginDaemonService>>,
    Json(payload): Json<SignRequest>,
) -> Result<Json<SignBox>, (StatusCode, String)> {
    // let sign = state
    //     .daemon
    //     .sign(payload.base64_url_data_string.clone())
    //     .expect("create signature failed");

    // Ok(Json(sign))

    todo!("校验签名");
}

async fn verify_handler(
    State(_state): State<Arc<PluginDaemonService>>,
    Json(payload): Json<VerifyRequest>,
) -> (StatusCode, Json<VerifyResponse>) {
    let sign_box = SignBox {
        public_key: payload.public_key,
        signature: payload.signature,
    };
    let success = sign_box
        .verify(payload.base64_url_data_string)
        .expect("verify signature failed");

    (StatusCode::OK, Json(VerifyResponse { success }))
}
