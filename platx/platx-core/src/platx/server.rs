use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

use crate::{
    plat::{Plat, StoreState},
    platx_config::PlatXConfig,
};

pub fn create_router(plugin_root: PathBuf, platx_config: PlatXConfig) -> anyhow::Result<Router> {
    let assets_root = platx_config.assets_root.clone();
    let assets_router = match url::Url::parse(&assets_root) {
        Ok(_) => Router::new(),
        Err(_) => {
            let assets_dir = ServeDir::new(plugin_root.join(assets_root.clone()))
                .not_found_service(ServeFile::new(
                    plugin_root.join(assets_root).join("index.html"),
                ));
            Router::new()
                .nest_service("/", assets_dir.clone())
                .fallback_service(assets_dir)
        }
    };

    let wasm_root = platx_config.wasm_root.clone();
    let wasm_router = match url::Url::parse(&wasm_root) {
        Ok(_) => Router::new(),
        Err(_) => Router::new().route("/invoke/:ty", post(invoke_handler)),
    };

    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.debug_info(true);

    let engine = Engine::new(&config).context("create engine failed")?;

    let mut linker: Linker<StoreState> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("add wasmtime wasi to linker failed")?;
    Plat::add_to_linker(&mut linker, |state| state).context("add plat to linker failed")?;

    let component = Component::from_file(&engine, plugin_root.join(platx_config.wasm_root).clone())
        .context("create component from file failed")?;

    let state = Arc::new(Mutex::new(ServerState {
        engine,
        linker,
        component,
        plugin_root: plugin_root.clone(),
    }));
    let router = Router::new()
        .merge(assets_router)
        .merge(wasm_router)
        .route(
            "/plugin/:scope/:name",
            post(|| async { "Extern plugin handler" }),
        )
        .with_state(state);

    Ok(router)
}

async fn invoke_handler(
    State(state): State<Arc<Mutex<ServerState>>>,
    Path(ty): Path<String>,
    body: String,
) -> String {
    let (world, mut store) = state.lock().unwrap().create_wasm().unwrap();

    world
        .call_emit(&mut store, &ty, &body)
        .expect("call reducer failed")
}

struct ServerState {
    engine: Engine,
    linker: Linker<StoreState>,
    component: Component,
    plugin_root: PathBuf,
}

impl ServerState {
    fn create_wasm(&self) -> anyhow::Result<(Plat, Store<StoreState>)> {
        let mut store = Store::new(&self.engine, StoreState::new(self.plugin_root.clone()));
        let world = Plat::instantiate(&mut store, &self.component, &self.linker).unwrap();

        Ok((world, store))
    }
}
