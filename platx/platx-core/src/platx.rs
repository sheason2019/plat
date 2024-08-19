use std::path::PathBuf;

use crate::plat::{Plat, StoreState};
use crate::platx_config::PlatXConfig;
use anyhow::Ok;
use axum::extract::{Path, State};
use axum::routing::post;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

pub struct PlatX {
    pub port: u16,
    pub config: PlatXConfig,
    directory: PathBuf,
}

impl PlatX {
    pub fn from_path(dir_path: PathBuf) -> anyhow::Result<Self> {
        let config = PlatXConfig::from_path(dir_path.clone())?;

        Ok(PlatX {
            config,
            port: 0,
            directory: dir_path,
        })
    }

    async fn create_wasm(&self) -> anyhow::Result<(Engine, Linker<StoreState>, Component)> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);
        config.debug_info(true);

        let engine = Engine::new(&config)?;

        let mut linker: Linker<StoreState> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let component =
            Component::from_file(&engine, self.directory.join(self.config.main.clone()))?;

        Ok((engine, linker, component))
    }

    pub async fn start_server(
        &mut self,
        listener: tokio::net::TcpListener,
    ) -> anyhow::Result<tokio::task::JoinHandle<()>> {
        let (engine, linker, component) = self.create_wasm().await?;

        let assets_dir = ServeDir::new(self.directory.join("assets"))
            .not_found_service(ServeFile::new(self.directory.join("assets/index.html")));

        let plugin_server = Router::new()
            .nest_service("/", assets_dir.clone())
            .fallback_service(assets_dir)
            .route("/invoke/:ty", post(invoke_handler))
            .route(
                "/plugin/:scope/:name",
                post(|| async { "Extern plugin handler" }),
            )
            .with_state((engine, linker, component, self.directory.clone()));

        self.port = listener.local_addr()?.port();

        let handler = tokio::spawn(async {
            axum::serve(listener, plugin_server)
                .await
                .expect("start server failed");
        });

        Ok(handler)
    }
}

async fn invoke_handler(
    State((engine, linker, component, plugin_dir)): State<(
        Engine,
        Linker<StoreState>,
        Component,
        PathBuf,
    )>,
    Path(ty): Path<String>,
    body: String,
) -> String {
    let mut store = Store::new(&engine, StoreState::new(plugin_dir));
    let world = Plat::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    world
        .call_action(&mut store, &ty, &body)
        .await
        .expect("call reducer failed")
}
