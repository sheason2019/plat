use core::profile::Profile;
use std::fs;
use tokio::sync::Mutex;

use tauri::{Manager, State};
use tracing::info;

pub mod core;

type PlatState<'a> = State<'a, Mutex<Profile>>;

#[tauri::command]
async fn get_profile(state: PlatState<'_>) -> Result<String, ()> {
    let profile_json = state.lock().await.to_json_string();
    Ok(profile_json)
}

#[tauri::command]
async fn create_isolate(state: PlatState<'_>, template: String) -> Result<String, ()> {
    let mut profile = state.lock().await;
    Ok(profile
        .generate_isolate()
        .await
        .expect("generate isolate failed"))
}

#[tauri::command]
async fn delete_isolate(state: PlatState<'_>, public_key: String) -> Result<(), ()> {
    let mut profile = state.lock().await;

    profile
        .delete_isolate(public_key)
        .await
        .expect("delete isolate failed");
    Ok(())
}

#[tauri::command]
async fn install_plugin(
    state: PlatState<'_>,
    public_key: String,
    plugin_file_path: String,
) -> Result<(), ()> {
    // let mut profile = state.lock().await;

    // let daemon = profile
    //     .daemon_services
    //     .iter_mut()
    //     .find(|i| i.plugin_daemon.public_key == public_key)
    //     .unwrap();

    // daemon
    //     .install_plugin(std::path::Path::new(plugin_file_path.as_str()).to_path_buf())
    //     .await
    //     .expect("install plugin failed");

    // Ok(())

    todo!()
}

#[tauri::command]
async fn delete_plugin(
    state: PlatState<'_>,
    public_key: String,
    plugin_name: String,
) -> Result<(), ()> {
    // let mut profile = state.lock().await;

    // let isolate = profile
    //     .daemon_services
    //     .iter_mut()
    //     .find(|i| i.plugin_daemon.public_key == public_key)
    //     .unwrap();

    // isolate
    //     .uninstall_plugin(plugin_name)
    //     .await
    //     .expect("remove plugin failed");

    // Ok(())

    todo!()
}

#[tauri::command]
async fn channel(state: PlatState<'_>, id: String, data: String) -> Result<(), ()> {
    state
        .lock()
        .await
        .app_util
        .complete_channel(
            id,
            serde_json::from_str(data.as_str()).expect("解析 JSON 数据失败"),
        )
        .await;

    Ok(())
}

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    tauri::async_runtime::block_on(async move {
        let data_root = handle.path().app_data_dir().expect("获取 AppData 目录失败");

        let log_dir = data_root.join(".log");
        if !log_dir.exists() {
            fs::create_dir_all(log_dir.clone()).expect("创建日志目录失败");
        }
        let file_appender = tracing_appender::rolling::daily(log_dir, "plat.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::fmt()
            .pretty()
            .with_writer(non_blocking)
            .with_ansi(false)
            .init();
        info!("日志模块已加载");

        info!("初始化用户信息");
        let profile = Profile::init(data_root.clone(), handle.clone())
            .await
            .expect("init profile failed");
        handle.manage(Mutex::new(profile));
        info!("用户信息初始化已完成");
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
            get_profile,
            create_isolate,
            delete_isolate,
            install_plugin,
            delete_plugin,
            channel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
