use flate2::{write::GzEncoder, Compression};

pub struct Tarer {
    dir: std::path::PathBuf,
}

impl Tarer {
    pub fn new(dir: std::path::PathBuf) -> Self {
        Tarer { dir }
    }

    pub fn tar(&self) -> anyhow::Result<()> {
        let tar_gz = std::fs::File::create(self.dir.join("output.platx"))?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(self.dir.clone(), self.dir.clone())?;

        Ok(())
    }
}

#[test]
fn test_builder_exec() {
    let builder = Tarer::new(std::path::Path::new(".").to_path_buf());
    builder.tar().unwrap();
}
