use host_service::{HostContext, HostService};
use std::sync::Arc;

use tauri::{Manager, State};

pub mod assets;
pub mod core;
pub mod host_service;

pub struct HostStateInner {
    pub discovery_service: HostService,
}
pub type HostState<'a> = State<'a, HostStateInner>;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    tauri::async_runtime::block_on(async move {
        // 启动宿主机服务
        let host_context = HostContext::new();
        let host_service = HostService::new_from_context(Arc::new(host_context))
            .await
            .unwrap();

        // 扫描文件系统构建资产树，并启动所有资产

        let state = HostStateInner {
            discovery_service: host_service,
        };

        handle.manage(state);
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(setup)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
