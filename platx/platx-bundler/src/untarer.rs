use std::{fs::File, path::PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

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

    pub fn untar_with_plugin_root(&self, plugin_root: PathBuf) -> anyhow::Result<()> {
        // 将插件内容解压至缓存路径
        let cache_path = plugin_root
            .join(".cache")
            .join(self.tar_file.file_name().unwrap());
        self.untar(cache_path)?;

        // TODO: 读取 plugin.json 文件，解析关键信息

        // TODO: 移动文件夹至指定目录，完成 plugin 的安装

        Ok(())
    }
}

#[test]
fn test_builder_exec() {
    let builder = Untarer::new(std::path::Path::new("./output.platx").to_path_buf());
    builder
        .untar(std::path::Path::new(".").join("./output"))
        .unwrap();
}
