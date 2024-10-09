use std::{borrow::Cow, ops::Deref, ptr, sync::Arc};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use serde_json::json;

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
    // 创建 Connection
    socket
        .send(Message::Text(serde_json::to_string(
            &json!({"type": "ok"}),
        )?))
        .await?;

    let connection = Arc::new(Connection::new());
    server.connections.lock().await.push(connection.clone());

    match connection.handle(socket, &server).await {
        Ok(_) => (),
        Err(e) => {
            println!("Connection 已断开，原因：{:?}", e);
        }
    }
    connection.stop().await;

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
