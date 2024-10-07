use std::str::FromStr;

use base64::Engine;
use sha3::Digest;
use tauri::Url;

#[tauri::command]
pub async fn open_daemon_window(app_handle: tauri::AppHandle, address: &str) -> Result<(), ()> {
    match open_daemon_window_inner(app_handle, address).await {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("open daemon window failed: {}", e.to_string());
            Err(())
        }
    }
}

async fn open_daemon_window_inner(
    app_handle: tauri::AppHandle,
    address: &str,
) -> anyhow::Result<()> {
    let daemon_url = Url::from_str(address)?;
    let label = {
        let address = urlencoding::encode(address).to_string();
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(address);
        let output = hasher.finalize();
        base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(output)
    };

    let builder = tauri::WebviewWindowBuilder::new(
        &app_handle,
        label,
        tauri::WebviewUrl::External(daemon_url),
    );
    let _ = builder.build()?;

    Ok(())
}
