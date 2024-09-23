use std::{borrow::Cow, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{CloseFrame, Message},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use plugin::models::PluginConfig;

use crate::service::PluginDaemonService;

pub async fn regist_handler(
    State(service): State<Arc<PluginDaemonService>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        socket
            .send(Message::Text("Ready".to_string()))
            .await
            .unwrap();

        let data = match socket.recv().await.unwrap().unwrap() {
            Message::Text(data) => data,
            _ => panic!("Message failed"),
        };
        let plugin_config: PluginConfig = serde_json::from_str(&data).unwrap();
        let name = plugin_config.name.clone();

        if service.registed_plugins.lock().await.contains_key(&name) {
            socket
                .send(Message::Close(Some(CloseFrame {
                    code: 1000,
                    reason: Cow::from("已存在相同名称的 Plugin"),
                })))
                .await
                .unwrap();
            return;
        }

        socket
            .send(Message::Text(service.plugin_daemon.public_key.clone()))
            .await
            .unwrap();
        match socket.recv().await.unwrap().unwrap() {
            Message::Text(public_key) => public_key,
            _ => {
                socket
                    .send(Message::Close(Some(CloseFrame {
                        code: 1000,
                        reason: Cow::from("消息类型错误"),
                    })))
                    .await
                    .unwrap();
                return;
            }
        };

        service
            .registed_plugins
            .lock()
            .await
            .insert(name.clone(), plugin_config);

        let (stop_sender, _rx) = tokio::sync::broadcast::channel::<()>(1);
        let (send_sender, _rx) = tokio::sync::broadcast::channel::<Message>(16);

        tokio::task::spawn({
            let send_sender = send_sender.clone();
            let stop_sender = stop_sender.clone();
            async move {
                let mut stop_sub = stop_sender.subscribe();
                let mut send_sub = send_sender.subscribe();
                loop {
                    tokio::select! {
                        message_option = socket.recv() => {
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
                        message_result = send_sub.recv() => {
                            match message_result {
                                Err(_) => break,
                                Ok(message) => {
                                    match socket.send(message).await {
                                        Ok(_) => (),
                                        Err(_e) => break,
                                    }
                                }
                            }
                        },
                        _ = tokio::time::sleep(Duration::from_secs(10)) => break,
                        _ = stop_sub.recv() => break,
                    }
                }

                let _ = stop_sender.send(());
            }
        });
        tokio::task::spawn({
            let stop_sender = stop_sender.clone();
            async move {
                let mut sub = stop_sender.subscribe();
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(4)) => {
                            send_sender.send(Message::Ping(Vec::new())).unwrap();
                        },
                        _ = sub.recv() => break,
                    }
                }

                let _ = stop_sender.send(());
            }
        });

        for connection in service.connections.lock().await.values() {
            let _ = connection.send_daemon(&service).await;
        }

        let _ = stop_sender.subscribe().recv().await;

        service.registed_plugins.lock().await.remove(&name);

        for connection in service.connections.lock().await.values() {
            let _ = connection.send_daemon(&service).await;
        }
    })
}
