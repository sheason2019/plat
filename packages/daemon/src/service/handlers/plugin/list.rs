use std::sync::Arc;

use axum::{extract::State, Json};
use plugin::models::Plugin;
use serde_json::{json, Value};

use crate::service::DaemonServer;

pub async fn list_plugin_handler(State(service): State<Arc<DaemonServer>>) -> Json<Value> {
    let registed_plugins = service.plugins.lock().await;
    let plugins: Vec<&Plugin> = registed_plugins.values().collect();
    let plugins = json!({
        "plugins": &plugins,
    });

    Json(plugins)
}
