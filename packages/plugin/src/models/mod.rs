use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub wasm_root: String,
    pub assets_root: String,
    pub storage_root: String,
    pub entries: Vec<PluginEntry>,
    pub address: Option<String>,
    pub daemon_address: Option<String>,
    pub regist_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginEntry {
    pub label: String,
    pub icon: String,
    pub href: String,
    pub target: String,
}
