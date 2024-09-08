use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use futures::future::BoxFuture;
use models::{PluginConfig, RegistedPlugin};
use serde_json::{json, Value};
use tokio::sync::{mpsc::Sender, Mutex};
use typings::{
    ConfirmSignatureHandler, RegistPluginRequest, ServiceState, SignRequest, VerifyRequest,
    VerifyResponse,
};

mod typings;

use crate::daemon::{PluginDaemon, SignBox};

pub struct PluginDaemonService {
    pub plugin_daemon: PluginDaemon,
    pub addr: String,
    pub registed_plugins: Arc<Mutex<HashMap<String, models::RegistedPlugin>>>,

    service_stop_sender: Sender<()>,
    confirm_signature_handler: Arc<ConfirmSignatureHandler>,
}

impl PluginDaemonService {
    pub async fn new(
        daemon: PluginDaemon,
        port: u16,
        confirm_signature_handler: impl Fn(SignRequest) -> BoxFuture<'static, anyhow::Result<bool>>
            + Send
            + Sync
            + 'static,
    ) -> anyhow::Result<Arc<Self>> {
        let confirm_signature_handler = Arc::new(confirm_signature_handler);

        let tcp_listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let addr = format!("http://{}", tcp_listener.local_addr()?.to_string());

        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

        let service = PluginDaemonService {
            plugin_daemon: daemon.clone(),
            addr,
            registed_plugins: Arc::new(Mutex::new(HashMap::new())),

            service_stop_sender: tx.clone(),
            confirm_signature_handler,
        };
        let service = Arc::new(service);

        tokio::task::spawn({
            let service = service.clone();
            async move {
                tokio::task::spawn({
                    let service = service.clone();
                    async move {
                        loop {
                            tokio::time::sleep(Duration::from_secs(5)).await;
                            service.health_check().await;
                        }
                    }
                });

                let app = Router::new()
                    .route("/", get(root_handler))
                    .route("/plugin", post(regist_plugin_handler))
                    .route("/sign", post(sign_handler))
                    .route("/verify", post(verify_handler))
                    .with_state(ServiceState {
                        daemon: service.plugin_daemon.clone(),
                        registed_plugins: service.registed_plugins.clone(),
                        confirm_signature_handler: service.confirm_signature_handler.clone(),
                    });
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

    pub async fn health_check(&self) {
        let mut rm_vec: Vec<String> = Vec::new();

        for (key, value) in self.registed_plugins.lock().await.iter() {
            let plugin_url = match url::Url::parse(&value.addr).unwrap().join("plugin.json") {
                Ok(value) => value,
                Err(_) => {
                    rm_vec.push(key.clone());
                    continue;
                }
            };
            let req = match reqwest::get(plugin_url).await {
                Ok(value) => value,
                Err(_) => {
                    rm_vec.push(key.clone());
                    continue;
                }
            };

            match req.error_for_status() {
                Ok(_) => (),
                Err(_) => {
                    rm_vec.push(key.clone());
                }
            }
        }

        for rm_key in rm_vec {
            self.registed_plugins.lock().await.remove(&rm_key);
        }
    }
}

async fn root_handler(State(state): State<ServiceState>) -> (StatusCode, Json<Value>) {
    let out = json!({
        "daemon": {
            "public_key": &state.daemon.public_key,
        },
        "plugins": state.registed_plugins.lock().await.deref(),
    });
    (StatusCode::OK, Json(out))
}

async fn regist_plugin_handler(
    State(state): State<ServiceState>,
    Json(payload): Json<RegistPluginRequest>,
) -> (StatusCode, String) {
    let addr = payload.addr;
    let target =
        url::Url::parse(&addr).expect(format!("parse addr {} as url failed", &addr).as_ref());

    let config = reqwest::get(target.join("plugin.json").unwrap())
        .await
        .expect("request regist plugin failed")
        .json::<PluginConfig>()
        .await
        .expect("json deserilize failed");

    let config_name = config.name.clone();

    // 如果 plugin 已存在则拒绝注册
    if state
        .registed_plugins
        .lock()
        .await
        .contains_key(&config_name)
    {
        return (
            StatusCode::CONFLICT,
            "已存在同名称的 Plugin，注册请求被拒绝".to_string(),
        );
    }

    let registed_plugin = RegistedPlugin {
        addr: addr.to_string(),
        config,
    };

    state
        .registed_plugins
        .lock()
        .await
        .insert(config_name.clone(), registed_plugin);

    (StatusCode::OK, "Plugin 注册成功".to_string())
}

async fn sign_handler(
    State(state): State<ServiceState>,
    Json(payload): Json<SignRequest>,
) -> Result<Json<SignBox>, (StatusCode, String)> {
    // TODO: 判断来源

    let handler = state.confirm_signature_handler.as_ref();
    let confirm_result = match handler(payload.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("执行签名验证逻辑失败，原因：{}", e),
            ))
        }
    };
    if !confirm_result {
        return Err((StatusCode::BAD_REQUEST, format!("签名验证被拒绝")));
    }

    let sign = state
        .daemon
        .sign(payload.base64_url_data_string.clone())
        .expect("create signature failed");

    Ok(Json(sign))
}

async fn verify_handler(
    State(_state): State<ServiceState>,
    Json(payload): Json<VerifyRequest>,
) -> (StatusCode, Json<VerifyResponse>) {
    // TODO:判断来源

    let sign_box = SignBox {
        public_key: payload.public_key,
        signature: payload.signature,
    };
    let success = sign_box
        .verify(payload.base64_url_data_string)
        .expect("verify signature failed");

    (StatusCode::OK, Json(VerifyResponse { success }))
}
