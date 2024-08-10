use core::profile::Profile;

pub mod core;

#[tauri::command]
fn get_profile() -> String {
    let profile = Profile::get_instance();
    serde_json::to_string(profile).unwrap()
}

#[tauri::command]
fn create_isolate(template: String) -> String {
    println!("create isolate with template {}", template);
    let profile = Profile::get_instance();
    let mut profile = profile.write().unwrap();

    profile.generate_isolate().expect("generate isolate failed")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_profile, create_isolate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
