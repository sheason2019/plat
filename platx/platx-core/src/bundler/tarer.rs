use std::path::PathBuf;

use flate2::{write::GzEncoder, Compression};

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
        for entry in self.dir.clone().read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if path.file_name().unwrap() == output_path.file_name().unwrap() {
                    continue;
                }
            }
            tar.append_path(path)?;
        }

        Ok(())
    }
}

#[test]
fn test_builder_exec() {
    let path = std::path::Path::new("./output.platx");
    let builder = Tarer::new(std::path::Path::new(".").to_path_buf());
    builder.tar(path.to_path_buf()).unwrap();
}
