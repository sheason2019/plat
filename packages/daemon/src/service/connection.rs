use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use x25519_dalek::{PublicKey, SharedSecret};

pub struct Connection {
    pub public_key: PublicKey,
    shared_secret: SharedSecret,
    send_channel: tokio::sync::mpsc::Sender<Message>,
    stop_sender: tokio::sync::broadcast::Sender<()>,
}

impl Connection {
    pub fn new(socket: WebSocket, public_key: PublicKey, shared_secret: SharedSecret) -> Self {
        let (mut tx, mut rx) = socket.split();
        let (send_tx, mut send_rx) = tokio::sync::mpsc::channel::<Message>(4);
        let (recv_tx, _) = tokio::sync::broadcast::channel::<Message>(16);
        let (stop_sender, _) = tokio::sync::broadcast::channel::<()>(4);

        // sender channel
        tokio::task::spawn({
            let stop_sender = stop_sender.clone();
            async move {
                let mut sub = stop_sender.subscribe();
                loop {
                    tokio::select! {
                        _ = sub.recv() => break,
                        _ = tokio::time::sleep(Duration::from_secs(10)) => break,
                        option = send_rx.recv() => {
                            match option {
                                None => break,
                                Some(message) => {
                                    match tx.send(message).await {
                                        Ok(_) => (),
                                        Err(_) => break,
                                    }
                                },
                            }
                        },
                    }
                }
                let _ = stop_sender.send(());
            }
        });

        // recv channel
        tokio::task::spawn({
            let recv_tx = recv_tx.clone();
            let stop_sender = stop_sender.clone();
            async move {
                let mut sub = stop_sender.subscribe();
                loop {
                    tokio::select! {
                        _ = sub.recv() => break,
                        item = rx.next() => {
                            match item {
                                None => break,
                                Some(result) => {
                                    match result {
                                        Err(_) => break,
                                        Ok(message) => {
                                            match recv_tx.send(message) {
                                                Ok(_) => (),
                                                Err(_) => break,
                                            }
                                        },
                                    }
                                }
                            }
                        },
                    }
                }

                let _ = stop_sender.send(());
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
                            send_tx.send(Message::Ping(Vec::new())).await.unwrap();
                        },
                    }
                }

                let _ = stop_sender.send(());
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

    pub async fn stop(&self) {
        let _ = self.stop_sender.send(());
    }
}
