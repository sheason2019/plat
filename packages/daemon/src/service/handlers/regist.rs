use std::{borrow::Cow, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{CloseFrame, Message},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use plugin::models::PluginConfig;

use crate::service::PluginDaemonService;

pub async fn regist_handler(
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
