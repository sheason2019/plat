use std::{
    fs::{self},
    path::Path,
};

use crate::core::isolate::Isolate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    isolates: Vec<Isolate>,
}

impl Profile {
    fn default() -> Self {
        Profile {
            isolates: Vec::new(),
        }
    }

    pub fn init() -> Self {
        let mut profile = Profile::default();
        // 从文件系统初始化 Profile 信息
        let read_dir = match fs::read_dir("./data") {
            Ok(value) => value,
            Err(_) => return Profile::default(),
        };
        for dir in read_dir {
            let dir = match dir {
                Ok(value) => value,
                Err(_) => continue,
            };

            let filename = match dir.file_name().into_string() {
                Ok(value) => value,
                Err(_) => continue,
            };
            if filename.starts_with(".") {
                continue;
            }

            let isolate_file = dir.path().join("isolate.json");
            if !isolate_file.exists() {
                continue;
            }
            let isolate: Isolate = match fs::read(isolate_file) {
                Ok(value) => match serde_json::from_slice(value.as_ref()) {
                    Ok(value) => value,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            profile.isolates.push(isolate)
        }

        profile
    }

    // 将 Profile 持久化保存到本地
    pub fn save(&self) {
        let data_path = Path::new("./data");
        if !data_path.exists() {
            fs::create_dir(data_path).expect(format!("create data dir error").as_ref());
        }

        for isolate in &self.isolates {
            let isolate_path = data_path.join(isolate.public_key.clone());
            if !isolate_path.exists() {
                fs::create_dir(isolate_path.clone()).expect(
                    format!(
                        "create isolate dir {} failed",
                        isolate_path.as_os_str().to_str().unwrap()
                    )
                    .as_ref(),
                );
            }

            let isolate_json_path = isolate_path.join("isolate.json");
            fs::write(
                isolate_json_path.clone(),
                serde_json::to_string(isolate).unwrap(),
            )
            .expect(
                format!(
                    "write {} failed",
                    isolate_json_path.as_os_str().to_str().unwrap()
                )
                .as_ref(),
            );
        }
    }

    pub fn generate_isolate(&mut self) -> Result<(), String> {
        let isolate = Isolate::generate()?;
        self.isolates.push(isolate);
        Ok(())
    }
}

#[test]
fn test_save() {
    let mut p = Profile::init();
    p.generate_isolate().expect("generate isolate failed");
    p.save();
}
