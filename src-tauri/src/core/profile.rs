use std::{
    fs::{self},
    path::Path,
};

use anyhow::Context;
use platx_core::platx::daemon::PlatXDaemon;

use crate::core::isolate::Isolate;

use super::dto::ProfileDTO;

pub struct Profile {
    pub isolates: Vec<Isolate>,
}

impl Profile {
    const fn new() -> Self {
        Profile {
            isolates: Vec::new(),
        }
    }

    pub async fn init() -> anyhow::Result<Self> {
        let mut profile = Profile::new();
        let profile_dto = ProfileDTO::from_fs()?;
        let data_root = std::path::Path::new("data");

        for isolate_dto in &profile_dto.isolates {
            let isolate_root = data_root.join(isolate_dto.public_key.clone());
            let mut isolate = Isolate {
                public_key: isolate_dto.public_key.clone(),
                private_key: isolate_dto.private_key.clone(),
                daemon: PlatXDaemon::new(),
            };
            isolate
                .init_plugin(isolate_root.join("plugins"))
                .await
                .context(format!(
                    "isolate {} init plugins failed",
                    isolate_dto.public_key.clone()
                ))?;

            profile.isolates.push(isolate);
        }

        Ok(profile)
    }

    // 将 Profile 持久化保存到本地
    pub fn save(&self) -> anyhow::Result<()> {
        let profile_dto = ProfileDTO::from_profile(self);
        profile_dto.save()?;

        Ok(())
    }

    pub fn as_dto(&self) -> ProfileDTO {
        ProfileDTO::from_profile(self)
    }

    pub async fn generate_isolate(&mut self) -> anyhow::Result<String> {
        let isolate = Isolate::generate().await?;
        let public_key = String::from(isolate.public_key.clone());

        self.isolates.push(isolate);
        self.save()?;
        Ok(public_key)
    }

    pub fn delete_isolate(&mut self, public_key: String) -> anyhow::Result<()> {
        // 在内存中删除 isolate
        let position = self
            .isolates
            .iter()
            .position(|item| item.public_key == public_key)
            .expect("cannot find position");
        self.isolates.remove(position);

        // 在文件系统中删除 isolate
        let p = Path::new("./data").join(public_key);
        fs::remove_dir_all(p)?;

        Ok(())
    }
}
