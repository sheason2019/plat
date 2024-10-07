use assets::host_assets::HostAssets;
use commands::{
    append_daemon, get_daemons, open_daemon_window, remove_daemon, update_daemon_password,
};
use tauri::{Emitter, Manager};
use tokio::sync::RwLock;
use typings::HostStateInner;

pub mod assets;
pub mod commands;
pub mod typings;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    handle.manage(HostStateInner {
        host_assets: RwLock::new(HostAssets::empty()),
    });

    tauri::async_runtime::block_on(async move {
        // 扫描文件系统构建资产树
        let host_assets = HostAssets::new_from_scan(handle)
            .await
            .expect("扫描本地资产失败");
        // 拉起资产树中定义的服务
        host_assets.up().await.unwrap();

        let state = handle.state::<HostStateInner>();
        let mut state_host_assets = state.host_assets.write().await;
        *state_host_assets = host_assets;
    });

    app.handle().emit("update-daemons", ())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            get_daemons,
            append_daemon,
            remove_daemon,
            update_daemon_password,
            open_daemon_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
