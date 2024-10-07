use std::{fs, path::PathBuf};

use anyhow::Context;
use flate2::{write::GzEncoder, Compression};
use plugin::models::Plugin;

pub fn tar(config_path: PathBuf, output_path: PathBuf) -> anyhow::Result<()> {
    let tar_gz = std::fs::File::create(output_path.clone())?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    // 读取 Plugin 配置
    let config_dir = config_path.parent().unwrap().to_path_buf();
    let config_bytes = fs::read(&config_path).context("读取 Plugin 配置文件失败")?;
    let config: Plugin =
        serde_json::from_slice(&config_bytes).context("反序列化 Plugin 配置失败")?;

    // 写入 WASM 文件
    tar.append_path_with_name(config_dir.join(&config.wasm_root), "plugin.wasm")
        .context("写入 WASM 二进制文件失败")?;

    // 写入静态资源文件夹
    let assets_dir = config_dir.join(&config.assets_root);
    if assets_dir.exists() {
        tar.append_dir_all("assets", assets_dir)
            .context("添加 assets 文件夹失败")?;
    }

    // 写入 Plugin 配置
    let plugin_config = {
        let mut new_config = config.clone();
        new_config.wasm_root = "plugin.wasm".to_string();
        new_config.assets_root = "assets".to_string();
        new_config.storage_root = "storage".to_string();
        new_config
    };

    let plugin_string =
        serde_json::to_string(&plugin_config).context("序列化 Plugin 字符串失败")?;
    let plugin_bytes = plugin_string.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(plugin_bytes.len().try_into()?);
    header.set_cksum();

    tar.append_data(&mut header, "plugin.json", plugin_bytes)
        .context("添加 Plugin 配置文件失败")?;

    Ok(())
}
