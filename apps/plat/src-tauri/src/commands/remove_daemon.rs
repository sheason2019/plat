use tauri::Emitter;

use crate::typings::HostState;

#[tauri::command]
pub async fn remove_daemon(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    daemon_key: &str,
) -> Result<(), ()> {
    match remove_daemon_inner(state, app_handle, daemon_key).await {
        Ok(val) => Ok(val),
        Err(e) => {
            println!("remove daemons error: {}", e);
            Err(())
        }
    }
}

async fn remove_daemon_inner(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    daemon_key: &str,
) -> anyhow::Result<()> {
    state
        .host_assets
        .delete_local_daemon(daemon_key.to_string())
        .await?;
    app_handle.emit("update-daemons", ())?;

    Ok(())
}
