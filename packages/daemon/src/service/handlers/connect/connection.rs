use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use plugin::models::Plugin;
use serde_json::json;
use tokio::{sync::broadcast::Sender, time};

use crate::service::DaemonServer;

pub struct Connection {
    terminate: Sender<()>,
    sender_channel: Sender<Message>,
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            terminate: Sender::new(4),
            sender_channel: Sender::new(4),
        }
    }

    pub async fn handle(
        &self,
        websocket: &mut WebSocket,
        server: &DaemonServer,
    ) -> anyhow::Result<()> {
        let mut terminate_sub = self.terminate.subscribe();
        let mut sender_sub = self.sender_channel.subscribe();

        tokio::spawn({
            let terminate = self.terminate.clone();
            let sender_channel = self.sender_channel.clone();
            async move {
                let mut terminate_sub = terminate.subscribe();
                loop {
                    tokio::select! {
                        _ = time::sleep(Duration::from_secs(5)) => {
                            sender_channel.send(Message::Ping(Vec::new())).expect("failed to send Ping message");
                        },
                        _ = terminate_sub.recv() => break,
                    }
                }
            }
        });

        self.send_daemon(server).await?;

        loop {
            tokio::select! {
                recv = websocket.recv() => {
                    let message = match recv {
                        None => break,
                        Some(value) => value?,
                    };
                    match message {
                        Message::Close(_) => break,
                        _ => ()
                    };
                },
                _ = time::sleep(Duration::from_secs(12)) => break,
                _ = terminate_sub.recv() => break,
                message = sender_sub.recv() => {
                    websocket.send(message?).await?;
                },
            }
        }

        Ok(())
    }

    pub async fn send_daemon(&self, server: &DaemonServer) -> anyhow::Result<()> {
        self.sender_channel
            .send(Message::Text(
                serde_json::to_string(&json!({
                    "type": "daemon",
                    "payload": {
                        "public_key": server.daemon.public_key,
                        "plugins": server.plugins.lock().await.values().collect::<Vec<&Plugin>>(),
                    },
                }))
                .context("serilize daemon json failed")?,
            ))
            .context("send message by channel failed")?;

        Ok(())
    }

    pub async fn stop(&self) {
        let _ = self.terminate.send(());
    }
}
