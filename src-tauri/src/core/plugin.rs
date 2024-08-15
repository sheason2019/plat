use crate::core::plat::{Plat, StoreState};

use axum::extract::State;
use axum::routing::post;
use axum::Router;
use std::{error::Error, fs, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub plugin: String,
    pub addr: String,
    entries: Vec<Entry>,

    #[serde(skip)]
    directory: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    name: String,
    href: String,
    target: String,
}

impl Plugin {
    pub fn load_by_path(plugin_dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let json_buf = fs::read(plugin_dir.join("plugin.json"))?;

        let mut plugin: Plugin = serde_json::from_slice(&json_buf)?;
        plugin.directory = plugin_dir.clone();
        Ok(plugin)
    }

    pub async fn create_server(
        &mut self,
    ) -> Result<(tokio::net::TcpListener, Router), Box<dyn Error>> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);
        config.debug_info(true);

        let engine = Engine::new(&config)?;

        let mut linker: Linker<StoreState> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let assets_dir = ServeDir::new(self.directory.join("assets"))
            .not_found_service(ServeFile::new(self.directory.join("assets/index.html")));

        let component = Component::from_file(&engine, self.directory.join(self.plugin.clone()))?;

        async fn invoke_handler(
            State((engine, linker, component)): State<(Engine, Linker<StoreState>, Component)>,
        ) -> String {
            let mut store = Store::new(&engine, StoreState::new());
            let world = Plat::instantiate_async(&mut store, &component, &linker)
                .await
                .unwrap();

            world
                .call_action(&mut store, "hello", "world")
                .await
                .expect("call reducer failed")
        }

        let plugin_server = Router::new()
            .nest_service("/", assets_dir.clone())
            .fallback_service(assets_dir)
            .route("/invoke", post(invoke_handler))
            .route(
                "/plugin/$scope/$name",
                post(|| async { "Extern plugin handler" }),
            )
            .with_state((engine, linker, component));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind listener failed");

        self.addr = listener.local_addr().unwrap().to_string();

        Ok((listener, plugin_server))
    }
}
