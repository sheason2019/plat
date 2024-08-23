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

use super::PlatXConfig;
use crate::plat::{Plat, StoreState};

pub fn create_router(plugin_root: PathBuf, platx_config: PlatXConfig) -> anyhow::Result<Router> {
    let assets_root = platx_config.assets_root.clone();

    let assets_path = plugin_root.join(assets_root.clone());
    let assets_dir = ServeDir::new(assets_path.clone())
        .not_found_service(ServeFile::new(assets_path.join("index.html")));

    let mut state = ServerState::new();
    state.with_wasm_context(plugin_root, platx_config)?;

    let router = Router::new()
        .nest_service("/", assets_dir.clone())
        .fallback_service(assets_dir)
        .route("/invoke/:ty", post(invoke_handler))
        .route(
            "/plugin/:scope/:name",
            post(|| async { "Extern plugin handler" }),
        )
        .with_state(Arc::new(Mutex::new(state)));

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
    wasm_context: Option<WasmContext>,
}

struct WasmContext {
    engine: Engine,
    linker: Linker<StoreState>,
    component: Component,
    plugin_root: PathBuf,
}

impl ServerState {
    fn new() -> Self {
        ServerState { wasm_context: None }
    }

    fn with_wasm_context(
        &mut self,
        plugin_root: PathBuf,
        platx_config: PlatXConfig,
    ) -> anyhow::Result<()> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);
        config.debug_info(true);

        let engine = Engine::new(&config).context("create engine failed")?;

        let mut linker: Linker<StoreState> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)
            .context("add wasmtime wasi to linker failed")?;
        Plat::add_to_linker(&mut linker, |state| state).context("add plat to linker failed")?;

        let component =
            Component::from_file(&engine, plugin_root.join(platx_config.wasm_root).clone())
                .context("create component from file failed")?;

        self.wasm_context = Some(WasmContext {
            engine,
            linker,
            component,
            plugin_root: plugin_root.clone(),
        });

        Ok(())
    }

    fn create_wasm(&self) -> anyhow::Result<(Plat, Store<StoreState>)> {
        let wasm_context = self.wasm_context.as_ref().unwrap();
        let mut store = Store::new(
            &wasm_context.engine,
            StoreState::new(wasm_context.plugin_root.clone()),
        );
        let world =
            Plat::instantiate(&mut store, &wasm_context.component, &wasm_context.linker).unwrap();

        Ok((world, store))
    }
}
