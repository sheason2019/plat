use std::{
    collections::HashMap,
    fs::{self},
    ops::Deref,
    path::Path,
};

use anyhow::Context;
use platx_core::platx::daemon::PlatXDaemon;
use serde_json::{json, Value};

use crate::core::isolate::Isolate;

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
        let mut isolates: Vec<Isolate> = Vec::new();
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

            let isolate_json: Value =
                serde_json::from_slice(std::fs::read(isolate_file)?.as_ref())?;

            let mut isolate = Isolate {
                public_key: isolate_json["public_key"].as_str().unwrap().to_string(),
                private_key: isolate_json["private_key"].as_str().unwrap().to_string(),
                daemon: PlatXDaemon::new(),
                plugin_handler_map: HashMap::new(),
            };

            isolate.daemon.start_server().await?;
            isolate
                .init_plugin(dir.path().join("plugins"))
                .await
                .context(format!(
                    "isolate {} init plugins failed",
                    isolate.public_key.clone()
                ))?;

            isolates.push(isolate);
        }

        Ok(Profile { isolates })
    }

    // 将 Profile 持久化保存到本地
    pub fn save(&self) -> anyhow::Result<()> {
        let data_root = Path::new("data");
        for isolate in &self.isolates {
            let isolate_path = data_root.join(&isolate.public_key).join("isolate.json");
            let isolate_json = json!({
                "public_key": &isolate.public_key,
                "private_key": &isolate.private_key,
            });
            fs::write(isolate_path, isolate_json.to_string())?;
        }

        Ok(())
    }

    pub fn to_json_string(&self) -> String {
        let mut isolates: Vec<Value> = Vec::new();
        for isolate in &self.isolates {
            let plugin_map = isolate.daemon.plugin_map.lock().unwrap();
            let isolate_json = json!({
                "public_key": &isolate.public_key,
                "private_key": &isolate.private_key,
                "plugins": plugin_map.deref(),
            });
            isolates.push(isolate_json);
        }

        let profile = json!({
            "isolates": &isolates,
        });

        serde_json::to_string(&profile).unwrap()
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
