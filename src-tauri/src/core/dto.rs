use std::path::Path;

use serde::{Deserialize, Serialize};

use super::profile::Profile;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileDTO {
    pub isolates: Vec<IsolateDTO>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IsolateDTO {
    pub public_key: String,
    pub private_key: String,
}

impl ProfileDTO {
    pub fn from_fs() -> anyhow::Result<Self> {
        let mut isolates: Vec<IsolateDTO> = Vec::new();
        let data_root = Path::new("data");
        let read_dir = std::fs::read_dir(data_root)?;
        for dir in read_dir {
            let dir = dir?;

            let filename = dir.file_name().into_string().unwrap();
            if filename.starts_with(".") {
                continue;
            }

            let isolate_file = dir.path().join("isolate.json");
            if !isolate_file.exists() {
                continue;
            }

            let isolate: IsolateDTO =
                serde_json::from_slice(std::fs::read(isolate_file)?.as_ref())?;

            isolates.push(isolate);
        }

        Ok(Self { isolates })
    }

    pub fn from_profile(profile: &Profile) -> Self {
        let mut isolates: Vec<IsolateDTO> = Vec::new();

        for isolate in &profile.isolates {
            let isolate_dto = IsolateDTO {
                public_key: isolate.public_key.clone(),
                private_key: isolate.private_key.clone(),
            };
            isolates.push(isolate_dto);
        }

        Self { isolates }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data_root = Path::new("data");
        for isolate in &self.isolates {
            let isolate_root = data_root.join(isolate.public_key.clone());
            if !isolate_root.exists() {
                std::fs::create_dir_all(isolate_root.clone())?;
            }

            let isolate_json_path = isolate_root.join("isolate.json");
            let isolate_json_bytes = serde_json::to_string(&self)?;
            std::fs::write(isolate_json_path, isolate_json_bytes)?;
        }

        Ok(())
    }

    pub fn to_json_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}
