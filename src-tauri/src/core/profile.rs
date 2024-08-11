use std::{
    fs::{self},
    path::Path,
    sync::{OnceLock, RwLock},
};

use crate::core::isolate::Isolate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    isolates: Vec<Isolate>,
}

impl Profile {
    const fn new() -> Self {
        Profile {
            isolates: Vec::new(),
        }
    }

    pub fn init() -> Self {
        let mut profile = Profile::new();
        // 从文件系统初始化 Profile 信息
        let read_dir = match fs::read_dir("./data") {
            Ok(value) => value,
            Err(_) => return Profile::new(),
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

    pub fn get_instance() -> &'static RwLock<Profile> {
        static INSTANCE: OnceLock<RwLock<Profile>> = OnceLock::new();
        INSTANCE.get_or_init(|| RwLock::new(Profile::init()))
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

    pub fn generate_isolate(&mut self) -> Result<String, String> {
        let isolate = Isolate::generate()?;
        let public_key = String::from(isolate.public_key.clone());

        self.isolates.push(isolate);
        self.save();
        Ok(public_key)
    }

    pub fn delete_isolate(&mut self, public_key: String) {
        // 在内存中删除 isolate
        let position = self
            .isolates
            .iter()
            .position(|item| item.public_key == public_key)
            .expect("cannot find position");
        self.isolates.remove(position);

        // 在文件系统中删除 isolate
        let p = Path::new("./data").join(public_key);
        fs::remove_dir_all(p).expect("remove isolate dir failed");
    }
}

#[test]
fn test_save() {
    let mut p = Profile::init();
    p.generate_isolate().expect("generate isolate failed");
}

#[test]
fn test_get_instance() {
    let instance_a = Profile::get_instance();
    let instance_b = Profile::get_instance();

    if !std::ptr::eq(instance_a, instance_b) {
        panic!("instance not equal");
    }
}
