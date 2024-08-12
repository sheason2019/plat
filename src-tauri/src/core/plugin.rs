use crate::core::plat::{Plat, StoreState};
use std::{error::Error, fs, path::PathBuf};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugin {
    name: String,
    plugin: String,

    #[serde(skip)]
    directory: PathBuf,
}

pub struct PluginRuntime {
    engine: Engine,
    plugin: Plugin,
}

impl Plugin {
    pub fn load_by_path(plugin_dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let json_buf = fs::read(plugin_dir.join("plugin.json"))?;

        let mut plugin: Plugin = serde_json::from_slice(&json_buf)?;
        plugin.directory = plugin_dir.clone();
        Ok(plugin)
    }
}

impl PluginRuntime {
    async fn create_world(&self) -> Result<(Plat, Store<StoreState>), Box<dyn Error>> {
        let engine = self.engine.clone();

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let mut store = Store::new(&engine, StoreState::new());

        let component = Component::from_file(
            &engine,
            self.plugin.directory.join(self.plugin.plugin.clone()),
        )?;

        let world = Plat::instantiate_async(&mut store, &component, &linker).await?;

        Ok((world, store))
    }

    pub async fn from_plugin(plugin: Plugin) -> Result<Self, Box<dyn Error>> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);
        config.debug_info(true);

        let engine = Engine::new(&config)?;

        Ok(PluginRuntime { engine, plugin })
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let (world, mut store) = self.create_world().await?;
        world.call_start(&mut store).await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_load_plugin() {
    let plugin_dir = std::path::Path::new(
        r"data\I5aV7bEC6dqmor1xVC31xadQm9Y2otocgEeVmvXbQtg=\plugins\plat\hello",
    );
    let plugin = Plugin::load_by_path(plugin_dir.to_path_buf()).expect("load plugin failed");
    let rt = PluginRuntime::from_plugin(plugin)
        .await
        .expect("create plugin runtime failed");

    rt.start().await.expect("start plugin failed");
}
