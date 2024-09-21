use std::{borrow::Cow, time::Duration};

use axum::extract::ws::{CloseFrame, Message, WebSocket};
use x25519_dalek::{PublicKey, SharedSecret};

pub struct Connection {
    pub public_key: PublicKey,
    shared_secret: SharedSecret,
    send_channel: tokio::sync::mpsc::Sender<Message>,
    stop_sender: tokio::sync::broadcast::Sender<String>,
}

impl Connection {
    pub fn new(mut socket: WebSocket, public_key: PublicKey, shared_secret: SharedSecret) -> Self {
        let (send_tx, mut send_rx) = tokio::sync::mpsc::channel::<Message>(4);
        let (recv_tx, _) = tokio::sync::broadcast::channel::<Message>(16);
        let (stop_sender, _) = tokio::sync::broadcast::channel::<String>(4);

        tokio::task::spawn({
            let stop_sender = stop_sender.clone();
            async move {
                let mut sub = stop_sender.subscribe();
                loop {
                    tokio::select! {
                        close_reason = sub.recv() => {
                            socket.send(Message::Close(Some(CloseFrame {
                                code: 1000,
                                reason: Cow::from(close_reason.unwrap()),
                            })))
                            .await
                            .unwrap();
                            break;
                        },
                        option = send_rx.recv() => {
                            match option {
                                None => break,
                                Some(message) => {
                                    match socket.send(message).await {
                                        Ok(_) => (),
                                        Err(e) => {
                                            let _ = stop_sender.send(format!("send err: {}", e));
                                        },
                                    }
                                },
                            }
                        },
                        item = socket.recv() => {
                            match item {
                                None => break,
                                Some(result) => {
                                    match result {
                                        Err(e) => {
                                            let _ = stop_sender.send(format!("recv err: {}", e));
                                        },
                                        Ok(message) => {
                                            let _ = recv_tx.send(message);
                                        },
                                    }
                                }
                            }
                        },
                    }
                }
            }
        });

        // ping channel
        tokio::task::spawn({
            let stop_sender = stop_sender.clone();
            let send_tx = send_tx.clone();
            async move {
                let mut sub = stop_sender.subscribe();
                loop {
                    tokio::select! {
                        _ = sub.recv() => break,
                        _ = tokio::time::sleep(Duration::from_secs(4)) => {
                            let _ = send_tx.send(Message::Ping(Vec::new())).await;
                        },
                    }
                }

                let _ = stop_sender.send("连接已超时".to_string());
            }
        });

        Connection {
            public_key,
            shared_secret,
            send_channel: send_tx,
            stop_sender,
        }
    }

    pub async fn wait(&self) {
        let _ = self.stop_sender.subscribe().recv().await;
    }

    pub async fn stop(&self, reason: &str) {
        let _ = self.stop_sender.send(reason.to_string());
    }
}
