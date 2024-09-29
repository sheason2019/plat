use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use anyhow::anyhow;
use base64::Engine;
use sha3::Digest;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tauri_plugin_fs::FsExt;

use super::daemon_asset::DaemonAsset;

pub struct TemplateAsset {
    pub path: PathBuf,
    pub sha3_256_string: String,
}

impl TemplateAsset {
    pub async fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        if !path.exists() {
            return Err(anyhow!("Template 文件不存在"));
        }

        let mut file = fs::File::open(&path)?;
        let mut hasher = sha3::Sha3_256::new();

        io::copy(&mut file, &mut hasher)?;

        let hash = hasher.finalize();
        let hash_string = base64::prelude::BASE64_URL_SAFE.encode(hash);

        Ok(TemplateAsset {
            path,
            sha3_256_string: hash_string,
        })
    }

    pub async fn new_from_default(app_handle: &AppHandle) -> anyhow::Result<Self> {
        let default_template_asset_path = app_handle
            .path()
            .resolve("default.temp.tar", BaseDirectory::Resource)
            .unwrap();
        let template_bytes = app_handle.fs().read(&default_template_asset_path)?;

        let default_template_path = app_handle
            .path()
            .data_dir()?
            .join("templates")
            .join("default.temp.tar");
        let templates_dir = default_template_path.parent().unwrap();
        if !templates_dir.exists() {
            fs::create_dir_all(templates_dir)?;
        }
        if default_template_path.exists() {
            fs::remove_file(&default_template_path)?;
        }

        let mut default_template_file = fs::File::create(&default_template_path)?;
        default_template_file.write_all(&template_bytes)?;

        Self::new_from_path(default_template_path).await
    }

    pub async fn reconciliation(&self, daemon_asset: &DaemonAsset) -> anyhow::Result<()> {
        let cur_assets_hash_file = daemon_asset.path.join("assets_sha3_256");
        if cur_assets_hash_file.exists() {
            let cur_assets_hash_string = fs::read_to_string(cur_assets_hash_file.clone())?;
            if cur_assets_hash_string == self.sha3_256_string {
                return Ok(());
            }
        }

        let daemon_assets_path = daemon_asset.path.join("assets");
        if daemon_assets_path.exists() {
            fs::remove_dir_all(&daemon_assets_path)?;
        }
        bundler::daemon::untar(self.path.clone(), daemon_assets_path)?;
        fs::write(cur_assets_hash_file, &self.sha3_256_string)?;

        return Ok(());
    }
}
