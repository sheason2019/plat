use std::sync::Arc;

use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::service::PluginDaemonService;

pub async fn connect_handler(
    State(service): State<Arc<PluginDaemonService>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {})
}
