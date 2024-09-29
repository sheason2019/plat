use std::path::PathBuf;

use flate2::{write::GzEncoder, Compression};

pub fn tar(source_dir: PathBuf, output_file: PathBuf) -> anyhow::Result<()> {
    let tar_gz = std::fs::File::create(output_file.clone())?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for entry in source_dir.read_dir()? {
        let entry = entry?;
        if entry.file_name().to_str() == output_file.file_name().unwrap().to_str() {
            continue;
        }

        if entry.path().is_dir() {
            tar.append_dir_all(entry.file_name(), entry.path())?;
        } else {
            tar.append_path(entry.path())?;
        }
    }

    Ok(())
}
