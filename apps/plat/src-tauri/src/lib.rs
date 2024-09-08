use core::profile::Profile;
use std::{fs, path::Path};
use tokio::sync::Mutex;

use tauri::{Manager, State};
use tracing::info;

pub mod core;

type PlatState<'a> = State<'a, Mutex<Profile>>;

#[tauri::command]
async fn get_profile(state: PlatState<'_>) -> Result<String, ()> {
    Ok(state.lock().await.to_json_string().await)
}

#[tauri::command]
async fn create_isolate(state: PlatState<'_>, _template: String) -> Result<String, ()> {
    let mut profile = state.lock().await;
    Ok(profile
        .generate_daemon_service()
        .await
        .expect("generate isolate failed"))
}

#[tauri::command]
async fn delete_isolate(state: PlatState<'_>, public_key: String) -> Result<(), ()> {
    let mut profile = state.lock().await;

    profile
        .delete_daemon_service(public_key)
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
    let app_util = state.lock().await.app_util.clone();

    let plugin_file_path = Path::new(&plugin_file_path);
    let plugin_directory = app_util
        .confirm_install_plugin_dialog(public_key.clone(), plugin_file_path.to_path_buf())
        .await
        .expect("parse plugin config failed");

    if plugin_directory.is_none() {
        return Ok(());
    }

    let plugin_directory = match plugin_directory {
        Some(value) => value,
        None => return Ok(()),
    };

    state
        .lock()
        .await
        .try_start_plugin_from_dir(&public_key, plugin_directory)
        .await
        .expect("remove plugin failed");

    Ok(())
}

#[tauri::command]
async fn delete_plugin(
    state: PlatState<'_>,
    public_key: String,
    plugin_name: String,
) -> Result<(), ()> {
    state
        .lock()
        .await
        .remove_plugin(&public_key, plugin_name)
        .await
        .expect("remove plugin failed");

    Ok(())
}

#[tauri::command]
async fn channel(state: PlatState<'_>, id: String, data: String) -> Result<(), ()> {
    state
        .lock()
        .await
        .app_util
        .clone()
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
