use std::{fs, path::PathBuf};

use anyhow::Ok;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default = "PlatXConfig::default")]
pub struct PlatXConfig {
    pub name: String,
    pub main: String,
    pub entries: Vec<PlatXEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatXEntry {
    pub label: String,
    pub icon: String,
    pub href: String,
    pub target: String,
}

impl PlatXConfig {
    fn default() -> Self {
        PlatXConfig {
            name: String::new(),
            main: "main.wasm".to_string(),
            entries: Vec::new(),
        }
    }
    pub fn from_path(dir_path: PathBuf) -> anyhow::Result<Self> {
        let plugin_file = dir_path.join("plugin.json");
        let plugin_bytes = fs::read(plugin_file)?;
        let config: PlatXConfig = serde_json::from_slice(&plugin_bytes)?;

        Ok(config)
    }
}
