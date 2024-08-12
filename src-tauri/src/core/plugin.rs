use crate::core::plat::{Plat, StoreState};
use std::{error::Error, fs, path::PathBuf};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    name: String,
    plugin: String,

    #[serde(skip)]
    runtime: Option<PluginRuntime>,
}

pub struct PluginRuntime {
    world: Plat,
    engine: Engine,
}

fn create_store(engine: &Engine) -> Store<StoreState> {
    Store::new(&engine, StoreState::new())
}

impl Plugin {
    pub async fn load_by_path(plugin_dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let json_buf = fs::read(plugin_dir.join("plugin.json"))?;

        let mut plugin: Plugin = serde_json::from_slice(&json_buf)?;

        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);
        config.debug_info(true);

        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let mut store = create_store(&engine);

        let component = Component::from_file(&engine, plugin_dir.join(plugin.plugin.clone()))?;

        let world = Plat::instantiate_async(&mut store, &component, &linker).await?;

        plugin.runtime = Some(PluginRuntime { world, engine });

        Ok(plugin)
    }

    async fn start(&self) -> Result<(), Box<dyn Error>> {
        let runtime = self.runtime.as_ref().unwrap();
        let mut store = create_store(&runtime.engine);

        runtime.world.call_start(&mut store).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_load_plugin() {
    let plugin_dir = std::path::Path::new(
        r"data\I5aV7bEC6dqmor1xVC31xadQm9Y2otocgEeVmvXbQtg=\plugins\plat\hello",
    );
    let plugin = Plugin::load_by_path(plugin_dir.to_path_buf())
        .await
        .expect("load plugin failed");

    plugin.start().await.expect("start plugin failed");
}
