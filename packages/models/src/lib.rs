use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistedPlugin {
    pub addr: String,
    pub config: PluginConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub version: String,
    pub wasm_root: String,
    pub entries: Vec<PluginEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginEntry {
    pub label: String,
    pub icon: String,
    pub href: String,
    pub target: String,
}

impl PluginConfig {
    pub fn from_file(file_path: PathBuf) -> anyhow::Result<Self> {
        let file_bytes = fs::read(file_path)?;
        let config: PluginConfig = serde_json::from_slice(&file_bytes)?;
        Ok(config)
    }
}
