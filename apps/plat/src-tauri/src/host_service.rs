use std::{collections::HashMap, sync::Arc};

use anyhow::Ok;
use axum::routing::get;
use daemon::daemon::PluginDaemon;
use tokio::{
    net::TcpListener,
    sync::{
        broadcast::{self, Sender},
        Mutex,
    },
};

pub struct HostContext {
    pub daemon_map: Mutex<HashMap<String, PluginDaemon>>,
}

impl HostContext {
    pub fn new() -> Self {
        HostContext {
            daemon_map: Mutex::new(HashMap::new()),
        }
    }
}

pub struct HostService {
    context: Arc<HostContext>,
    address: String,
    stop_sender: Sender<()>,
}

impl HostService {
    pub async fn new_from_context(context: Arc<HostContext>) -> anyhow::Result<Self> {
        let tcp_listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = format!("http://{}", tcp_listener.local_addr()?);

        let (tx, _rx) = broadcast::channel::<()>(4);
        tokio::task::spawn({
            let tx = tx.clone();
            async move {
                let mut sub = tx.subscribe();
                let app = axum::Router::new().route("/api/regist", get(|| async { "" }));
                axum::serve(tcp_listener, app)
                    .with_graceful_shutdown(async move {
                        let _ = sub.recv().await;
                    })
                    .await
                    .unwrap();
            }
        });

        Ok(HostService {
            context,
            address,
            stop_sender: tx.clone(),
        })
    }

    pub fn stop(&self) {
        self.stop_sender.send(()).unwrap();
    }
}
