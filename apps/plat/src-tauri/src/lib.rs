use assets::host_assets::{self, HostAssets};
use tauri::{Manager, State};

pub mod assets;
pub mod core;

pub struct HostStateInner {
    host_assets: HostAssets,
}
pub type HostState<'a> = State<'a, HostStateInner>;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    tauri::async_runtime::block_on(async move {
        
        // 扫描文件系统构建资产树
        let host_assets = HostAssets::new_from_scan(handle)
            .await
            .expect("扫描本地资产失败");
        // 启动所有资产

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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
