use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    daemon::SignBox,
    service::{typings::SignRequest, DaemonServer},
};

pub async fn sig_handler(
    State(state): State<Arc<DaemonServer>>,
    Json(payload): Json<SignRequest>,
) -> Result<Json<SignBox>, (StatusCode, String)> {
    let sign = state
        .plugin_daemon
        .sign(payload.base64_url_data_string.clone())
        .expect("create signature failed");

    Ok(Json(sign))
}
