use std::sync::Arc;

use axum::extract::State;

use crate::service::DaemonServer;

pub async fn delete_plugin_handler(State(service): State<Arc<DaemonServer>>) {
    // 从 Plugins 中读取需要删除的插件
    // 获取 Connection，请求用户确认
    // 根据返回的结果删除或取消删除插件
    // 尝试刷新 assets
}
