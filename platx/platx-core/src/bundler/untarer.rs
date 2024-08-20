use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Context};
use flate2::read::GzDecoder;
use tar::Archive;

use crate::platx_config::PlatXConfig;

pub struct Untarer {
    tar_file: std::path::PathBuf,
}

impl Untarer {
    pub fn new(tar_file: std::path::PathBuf) -> Self {
        Untarer { tar_file }
    }

    pub fn untar(&self, out_dir: PathBuf) -> anyhow::Result<()> {
        let tar_gz = File::open(self.tar_file.clone())?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(out_dir)?;

        Ok(())
    }

    pub fn untar_with_plugin_root(&self, plugin_root: PathBuf) -> anyhow::Result<PathBuf> {
        // 将插件内容解压至缓存路径
        let cache_path = plugin_root
            .join(".cache")
            .join(self.tar_file.file_name().unwrap());
        self.untar(cache_path.clone())?;

        // 读取 plugin.json 文件，解析关键信息
        let config_bytes = std::fs::read(cache_path.clone().join("plugin.json"))?;
        let config: PlatXConfig = serde_json::from_slice(&config_bytes)?;
        let name_split: Vec<&str> = config.name.split("/").collect();
        if name_split.len() != 2 {
            return Err(anyhow!("不规范的插件名称：{}", config.name.clone()));
        }

        // 移动文件夹至指定目录，完成 plugin 的安装
        let scope = urlencoding::encode(name_split[0]).to_string();
        let name = urlencoding::encode(name_split[1]).to_string();

        let scope_dir = plugin_root.clone().join(scope);
        if !scope_dir.exists() {
            std::fs::create_dir_all(scope_dir.clone())
                .with_context(|| format!("create scope dir {:?} failed", scope_dir.clone()))?;
        }

        let plugin_dir = scope_dir.clone().join(name);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(plugin_dir.clone())?;
        }

        std::fs::rename(cache_path, plugin_dir.clone())?;

        Ok(plugin_dir)
    }
}
