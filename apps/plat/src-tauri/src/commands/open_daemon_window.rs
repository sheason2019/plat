use std::str::FromStr;

use base64::Engine;
use sha3::Digest;
use tauri::{ipc::CapabilityBuilder, Listener, Manager, Url};

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
        hex::encode(&output[0..16])
    };

    let builder = tauri::WebviewWindowBuilder::new(
        &app_handle,
        format!("daemon-{}", label),
        tauri::WebviewUrl::External(daemon_url),
    );

    let window = builder.build()?;

    Ok(())
}
