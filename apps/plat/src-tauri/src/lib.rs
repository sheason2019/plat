use assets::host_assets::HostAssets;
use commands::{append_daemon, get_daemons, remove_daemon};
use tauri::Manager;
use typings::HostStateInner;

pub mod assets;
pub mod commands;
pub mod typings;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    tauri::async_runtime::block_on(async move {
        // 扫描文件系统构建资产树
        let host_assets = HostAssets::new_from_scan(handle)
            .await
            .expect("扫描本地资产失败");
        // 拉起资产树中定义的服务
        host_assets.up().await.unwrap();

        let state = HostStateInner { host_assets };

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
        .invoke_handler(tauri::generate_handler![
            get_daemons,
            append_daemon,
            remove_daemon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
