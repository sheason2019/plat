use std::path::PathBuf;

use flate2::{write::GzEncoder, Compression};
use models::PluginConfig;

pub struct Tarer {
    dir: std::path::PathBuf,
}

impl Tarer {
    pub fn new(dir: std::path::PathBuf) -> Self {
        Tarer { dir }
    }

    pub fn tar(&self, output_path: PathBuf) -> anyhow::Result<()> {
        let tar_gz = std::fs::File::create(output_path.clone())?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        let config_path = self.dir.join("plugin.json");
        let config = PluginConfig::from_file(config_path.clone())?;
        tar.append_path(config_path)?;

        tar.append_path(self.dir.join(&config.wasm_root))?;
        let assets_dir = self.dir.join("assets");
        if assets_dir.exists() {
            tar.append_dir_all("assets", assets_dir)?;
        }

        Ok(())
    }
}

#[test]
fn test_builder_exec() {
    let path = std::path::Path::new("./output.plat");
    let builder = Tarer::new(std::path::Path::new(".").to_path_buf());
    builder.tar(path.to_path_buf()).unwrap();
}
