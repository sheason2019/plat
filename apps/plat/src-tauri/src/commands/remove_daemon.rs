use tauri::Emitter;

use crate::typings::HostState;

#[tauri::command]
pub async fn remove_daemon(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    public_key: Option<&str>,
    address: Option<&str>,
) -> Result<(), ()> {
    match remove_daemon_inner(state, app_handle, public_key, address).await {
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
    public_key: Option<&str>,
    address: Option<&str>,
) -> anyhow::Result<()> {
    match public_key {
        Some(public_key) => {
            state
                .host_assets
                .read()
                .await
                .delete_local_daemon(public_key.to_string())
                .await?;
        }
        None => (),
    }

    match address {
        Some(address) => {
            state
                .host_assets
                .read()
                .await
                .delete_remote_daemon(address.to_string())
                .await?;
        }
        None => (),
    }

    app_handle.emit("update-daemons", ())?;

    Ok(())
}
