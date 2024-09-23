use base64::prelude::*;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::{borrow::Cow, sync::Arc};
use x25519_dalek::{EphemeralSecret, PublicKey};

use axum::{
    extract::{
        ws::{CloseFrame, Message},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

use crate::service::{connection::Connection, PluginDaemonService};

pub async fn connect_handler(
    State(service): State<Arc<PluginDaemonService>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let secret = EphemeralSecret::random_from_rng(OsRng);
        // 交换 x25519 密钥，并验证用户密码
        socket
            .send(Message::Text(
                BASE64_URL_SAFE.encode(PublicKey::from(&secret).as_bytes()),
            ))
            .await
            .unwrap();

        let message = socket.recv().await.unwrap().unwrap();
        let connect_content: ConnectContent = match message {
            Message::Text(val) => match serde_json::from_str(&val) {
                Ok(val) => val,
                Err(_) => {
                    socket
                        .send(Message::Close(Some(CloseFrame {
                            code: 1000,
                            reason: Cow::from("序列化 ConnectContent 失败"),
                        })))
                        .await
                        .unwrap();
                    return;
                }
            },
            _ => return,
        };
        let connect_public_key: [u8; 32] = BASE64_URL_SAFE
            .decode(connect_content.public_key)
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();
        let connect_public_key = PublicKey::from(connect_public_key);
        let shared_secret = secret.diffie_hellman(&connect_public_key);

        let name = BASE64_URL_SAFE.encode(connect_public_key.as_bytes());

        let mut hasher = Sha3_256::new();
        hasher.update(shared_secret.as_bytes());
        hasher.update(service.plugin_daemon.password.as_bytes());

        let password_hash = hasher.finalize();
        let client_password_hash = BASE64_URL_SAFE
            .decode(connect_content.password_hash)
            .unwrap();
        if !compare_slice(&password_hash, &client_password_hash) {
            socket
                .send(Message::Close(Some(CloseFrame {
                    code: 1000,
                    reason: Cow::from("密码验证失败"),
                })))
                .await
                .unwrap();
            return;
        }

        // 创建 Connection
        socket.send(Message::Text("OK".to_string())).await.unwrap();
        let connection = Connection::new(socket, connect_public_key, shared_secret);
        let connection = Arc::new(connection);

        // 发送 daemon 信息
        connection.send_daemon(&service).await.unwrap();

        // 将 Connection 的引用写入 HashMap
        service
            .connections
            .lock()
            .await
            .insert(name.clone(), connection.clone());

        // 等待 Connection 结束
        connection.wait().await;

        service.connections.lock().await.remove(&name);
    })
}

#[derive(Serialize, Deserialize)]
struct ConnectContent {
    public_key: String,
    password_hash: String,
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
