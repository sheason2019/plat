use anyhow::anyhow;
use daemon::daemon::{PluginDaemon, PluginDaemonVariant};
use tauri::Emitter;

use crate::typings::HostState;

#[tauri::command]
pub async fn append_daemon(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    variant: &str,
    remote_address: &str,
) -> Result<(), ()> {
    match append_daemon_inner(state, app_handle, variant, remote_address).await {
        Ok(val) => Ok(val),
        Err(e) => {
            println!("append command error: {}", e);
            Err(())
        }
    }
}

async fn append_daemon_inner(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    variant: &str,
    remote_address: &str,
) -> anyhow::Result<()> {
    match variant {
        "local-generate" => {
            let plugin_daemon = PluginDaemon::new_random()?;
            state
                .host_assets
                .append_daemon(&app_handle, plugin_daemon)
                .await?;
            app_handle.emit("update-daemons", ())?;
        }
        "remote" => {
            let mut plugin_daemon = PluginDaemon::default();
            plugin_daemon.address = Some(remote_address.to_string());
            plugin_daemon.variant = PluginDaemonVariant::Remote;
            state
                .host_assets
                .append_daemon(&app_handle, plugin_daemon)
                .await?;
            app_handle.emit("update-daemons", ())?;
        }
        _ => return Err(anyhow!("创建账号的模式超出预期")),
    }

    Ok(())
}
