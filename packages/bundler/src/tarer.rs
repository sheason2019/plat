use std::{fs, path::PathBuf};

use flate2::{write::GzEncoder, Compression};
use plugin::models::PluginConfig;

pub struct Tarer {
    config_path: std::path::PathBuf,
}

impl Tarer {
    pub fn new(dir: std::path::PathBuf) -> Self {
        Tarer { config_path: dir }
    }

    pub fn tar(&self, output_path: PathBuf) -> anyhow::Result<()> {
        let tar_gz = std::fs::File::create(output_path.clone())?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        // 读取 Plugin 配置
        let config_path = self.config_path.join("plugin.json");
        let config_bytes = fs::read(&config_path)?;
        let config: PluginConfig = serde_json::from_slice(&config_bytes)?;

        // 写入 WASM 文件
        tar.append_path_with_name(self.config_path.join(&config.wasm_root), "plugin.wasm")?;

        // 写入静态资源文件夹
        let assets_dir = self.config_path.join(&config.assets_root);
        if assets_dir.exists() {
            tar.append_dir_all("assets", assets_dir)?;
        }

        // 写入 Plugin 配置
        let plugin_config = {
            let mut new_config = config.clone();
            new_config.wasm_root = "plugin.wasm".to_string();
            new_config.assets_root = "assets".to_string();
            new_config.storage_root = "storage".to_string();
            new_config
        };

        let plugin_string = serde_json::to_string(&plugin_config)?;
        let plugin_bytes = plugin_string.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(plugin_bytes.len().try_into()?);
        header.set_cksum();

        tar.append_data(&mut header, "plugin.json", plugin_bytes)?;

        Ok(())
    }
}
