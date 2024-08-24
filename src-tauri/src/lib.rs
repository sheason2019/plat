use core::profile::Profile;
use tokio::sync::Mutex;

use tauri::{Manager, State};

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
        .expect("delete isolate failed");
    Ok(())
}

#[tauri::command]
async fn install_plugin(
    state: PlatState<'_>,
    public_key: String,
    plugin_file_path: String,
) -> Result<(), ()> {
    let mut profile = state.lock().await;

    let isolate = profile
        .isolates
        .iter_mut()
        .find(|i| i.public_key == public_key)
        .unwrap();

    isolate
        .install_plugin(std::path::Path::new(plugin_file_path.as_str()).to_path_buf())
        .await
        .expect("install plugin failed");

    Ok(())
}

#[tauri::command]
async fn delete_plugin(
    state: PlatState<'_>,
    public_key: String,
    plugin_name: String,
) -> Result<(), ()> {
    let mut profile = state.lock().await;

    let isolate = profile
        .isolates
        .iter_mut()
        .find(|i| i.public_key == public_key)
        .unwrap();

    isolate
        .uninstall_plugin(plugin_name)
        .await
        .expect("remove plugin failed");

    Ok(())
}

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    tauri::async_runtime::block_on(async move {
        let profile = Profile::init().await.expect("init profile failed");
        handle.manage(Mutex::new(profile));
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
