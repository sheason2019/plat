use anyhow::anyhow;
use tauri::Emitter;

use crate::typings::HostState;

#[tauri::command]
pub async fn update_daemon_password(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    daemon_key: &str,
    new_password: &str,
) -> Result<(), ()> {
    match update_daemon_password_inner(state, app_handle, daemon_key, new_password).await {
        Ok(val) => Ok(val),
        Err(e) => {
            println!("update daemons password error: {}", e);
            Err(())
        }
    }
}

async fn update_daemon_password_inner(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    daemon_key: &str,
    new_password: &str,
) -> anyhow::Result<()> {
    match state
        .host_assets
        .read()
        .await
        .local_daemons
        .lock()
        .await
        .get_mut(&daemon_key.to_string())
    {
        None => return Err(anyhow!("未找到对应的 Daemon")),
        Some(daemon_asset) => {
            daemon_asset
                .update_password(new_password.to_string())
                .await?;
            app_handle.emit("update-daemons", ())?;
        }
    }

    Ok(())
}
