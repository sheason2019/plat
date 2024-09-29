use std::{fs::File, path::PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

pub fn untar(tar_file: PathBuf, out_dir: PathBuf) -> anyhow::Result<()> {
    let tar_gz = File::open(tar_file.clone())?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(out_dir)?;

    Ok(())
}
