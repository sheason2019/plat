use std::{
    fs::{self},
    path::Path,
    sync::{Arc, Mutex, OnceLock, RwLock},
};

use crate::core::isolate::Isolate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    isolates: Vec<Arc<Mutex<Isolate>>>,
}

impl Profile {
    const fn new() -> Self {
        Profile {
            isolates: Vec::new(),
        }
    }

    pub async fn init() -> Self {
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

            let filename = dir
                .file_name()
                .into_string()
                .expect("dir name into string failed");
            if filename.starts_with(".") {
                continue;
            }

            let isolate_file = dir.path().join("isolate.json");
            if !isolate_file.exists() {
                continue;
            }

            let isolate: Isolate =
                serde_json::from_slice(fs::read(isolate_file).unwrap().as_ref()).unwrap();

            let isolate = Arc::new(Mutex::new(isolate));
            Isolate::init_plugin(Arc::clone(&isolate), dir.path().join("plugins")).await;

            profile.isolates.push(Arc::clone(&isolate));
        }

        profile.save();

        profile
    }

    pub async fn get_instance() -> &'static RwLock<Profile> {
        static INSTANCE: OnceLock<RwLock<Profile>> = OnceLock::new();
        if INSTANCE.get().is_none() {
            let _ = INSTANCE.set(RwLock::new(Profile::init().await));
        }

        INSTANCE.get().expect("get profile instance failed")
    }

    // 将 Profile 持久化保存到本地
    pub fn save(&self) {
        let data_path = Path::new("./data");
        if !data_path.exists() {
            fs::create_dir(data_path).expect(format!("create data dir error").as_ref());
        }

        for isolate in &self.isolates {
            let isolate = isolate.lock().unwrap();
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

            let mut isolate = isolate.clone();
            isolate.plugins = Vec::new();

            let isolate_json_path = isolate_path.join("isolate.json");
            fs::write(
                isolate_json_path.clone(),
                serde_json::to_string(&isolate.clone()).unwrap(),
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

        self.isolates.push(Arc::new(Mutex::new(isolate)));
        self.save();
        Ok(public_key)
    }

    pub fn delete_isolate(&mut self, public_key: String) {
        // 在内存中删除 isolate
        let position = self
            .isolates
            .iter()
            .position(|item| item.lock().unwrap().public_key == public_key)
            .expect("cannot find position");
        self.isolates.remove(position);

        // 在文件系统中删除 isolate
        let p = Path::new("./data").join(public_key);
        fs::remove_dir_all(p).expect("remove isolate dir failed");
    }
}

#[tokio::test]
async fn test_save() {
    let mut p = Profile::init().await;
    p.generate_isolate().expect("generate isolate failed");
}

#[tokio::test]
async fn test_get_instance() {
    let instance_a = Profile::get_instance().await;
    let instance_b = Profile::get_instance().await;

    if !std::ptr::eq(instance_a, instance_b) {
        panic!("instance not equal");
    }
}
