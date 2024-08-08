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
    pub fn init() -> Self {
        match fs::read("./data/plat-profile.json") {
            Ok(value) => match serde_json::from_slice(value.as_ref()) {
                Ok(value) => return value,
                Err(_) => (),
            },
            Err(_) => (),
        }

        Profile {
            isolates: Vec::new(),
        }
    }

    // 将 Profile 持久化保存到本地
    pub fn save(&self) {
        let json_val = serde_json::to_string(self).unwrap();

        if !Path::new("./data").exists() {
            fs::create_dir("./data").unwrap();
        }
        fs::write("./data/plat-profile.json", json_val).unwrap();
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
    p.generate_isolate();
    p.save();
}
