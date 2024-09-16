use crate::typings::HostState;
use plugin::models::PluginConfig;
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
    for daemon in state.host_assets.daemons.lock().await.values() {
        let plugin_map = daemon.plugins.lock().await;
        let plugin_daemon = daemon.get_plugin_daemon().await;
        let plugins: Vec<&PluginConfig> = plugin_map.values().map(|i| &i.plugin_config).collect();
        daemons.push(json!({
            "daemon": plugin_daemon,
            "plugins": &plugins,
        }));
    }

    Ok(serde_json::to_string(&daemons)?)
}
