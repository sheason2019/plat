use core::profile::Profile;

pub mod core;

#[tauri::command]
async fn get_profile() -> String {
    let profile = Profile::get_instance().await;
    let profile_dto = profile.lock().unwrap().as_dto();

    serde_json::to_string(&profile_dto).expect("profile to dto json failed")
}

#[tauri::command]
async fn create_isolate(template: String) -> String {
    let profile = Profile::get_instance().await;
    let mut profile = profile.lock().unwrap();

    profile.generate_isolate().expect("generate isolate failed")
}

#[tauri::command]
async fn delete_isolate(public_key: String) {
    let profile = Profile::get_instance().await;
    let mut profile = profile.lock().unwrap();

    profile
        .delete_isolate(public_key)
        .expect("delete isolate failed");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_profile,
            create_isolate,
            delete_isolate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
