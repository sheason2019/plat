use anyhow::anyhow;
use daemon::daemon::PluginDaemon;
use tauri::Emitter;

use crate::typings::HostState;

#[tauri::command]
pub async fn append_daemon(
    state: HostState<'_>,
    app_handle: tauri::AppHandle,
    variant: &str,
) -> Result<(), ()> {
    match append_daemon_inner(state, app_handle, variant).await {
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
) -> anyhow::Result<()> {
    match variant {
        "local-generate" => {
            let plugin_daemon = PluginDaemon::new()?;
            state.host_assets.append_daemon(plugin_daemon).await?;
            app_handle.emit("update-daemons", ())?;
        }
        _ => return Err(anyhow!("创建账号的模式超出预期")),
    }

    Ok(())
}
