use std::{fs::File, path::PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

pub struct Untarer {
    tar_file: PathBuf,
}

impl Untarer {
    pub fn new(tar_file: PathBuf) -> Self {
        Untarer { tar_file }
    }

    pub fn untar(&self, out_dir: PathBuf) -> anyhow::Result<()> {
        let tar_gz = File::open(self.tar_file.clone())?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(out_dir)?;

        Ok(())
    }
}
