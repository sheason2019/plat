use anyhow::anyhow;
use base64::prelude::*;
use rand::Rng;
use sha3::{Digest, Sha3_256};
use std::{borrow::Cow, ops::Deref, ptr, sync::Arc};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

use crate::service::DaemonServer;

use super::Connection;

pub async fn connect_handler(
    State(server): State<Arc<DaemonServer>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        match handle_connection(&mut socket, server).await {
            Ok(()) => (),
            Err(e) => {
                let _ = socket
                    .send(Message::Close(Some(CloseFrame {
                        code: 1000,
                        reason: Cow::from(e.to_string()),
                    })))
                    .await;
            }
        };
    })
}

async fn handle_connection(
    socket: &mut WebSocket,
    server: Arc<DaemonServer>,
) -> anyhow::Result<()> {
    // 通过时间戳计算盐值
    let salt_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    socket.send(Message::Text(salt_string.clone())).await?;

    let message = socket.recv().await.unwrap()?;
    let client_password_string = message.to_text()?;
    let client_password_hash = BASE64_URL_SAFE.decode(client_password_string)?;

    let mut hasher = Sha3_256::new();
    hasher.update(server.daemon.password.as_bytes());
    hasher.update(salt_string.as_bytes());
    let server_password_hash = hasher.finalize();

    if !compare_slice(&server_password_hash, &client_password_hash) {
        return Err(anyhow!("密码验证失败"));
    }

    // 创建 Connection
    socket.send(Message::Text("OK".to_string())).await.unwrap();

    let connection = Arc::new(Connection::new());
    server.connections.lock().await.push(connection.clone());
    connection.handle(socket).await?;

    let deref = connection.deref();
    let position = server
        .connections
        .lock()
        .await
        .iter()
        .position(|v| ptr::eq(v.deref(), deref));
    match position {
        None => (),
        Some(val) => {
            let _ = server.connections.lock().await.remove(val);
        }
    }

    Ok(())
}

fn compare_slice(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
}
