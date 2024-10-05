use crate::typings::HostState;
use serde_json::json;

#[tauri::command]
pub async fn get_daemons(state: HostState<'_>) -> Result<String, ()> {
    match get_daemons_inner(state).await {
        Ok(val) => Ok(val),
        Err(e) => {
            println!("get daemons error: {}", e);
            Err(())
        }
    }
}

async fn get_daemons_inner(state: HostState<'_>) -> anyhow::Result<String> {
    let mut daemons: Vec<serde_json::Value> = Vec::new();
    for daemon in state.host_assets.local_daemons.lock().await.values() {
        let plugin_daemon = daemon.to_json_string().await?;
        daemons.push(json!({
            "daemon": plugin_daemon,
        }));
    }

    Ok(serde_json::to_string(&daemons)?)
}
