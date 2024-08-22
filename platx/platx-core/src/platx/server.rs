use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use axum::{
    extract::{Path, Request, State},
    routing::{any, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};
use url::Url;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

use crate::{
    plat::{Plat, StoreState},
    platx_config::PlatXConfig,
};

pub fn create_router(plugin_root: PathBuf, platx_config: PlatXConfig) -> anyhow::Result<Router> {
    let assets_root = platx_config.assets_root.clone();
    let assets_router = match url::Url::parse(&assets_root) {
        Ok(assets_proxy_root) => {
            println!("proxy assets to url: {}", assets_proxy_root.to_string());
            let handler = any(
                |State(state): State<Arc<Mutex<ServerState>>>, req: Request| async {
                    reverse_proxy_handler(state, req, assets_proxy_root).await
                },
            );
            Router::new()
                .route("/*asset_path", handler.clone())
                .route("/", handler)
        }
        Err(_) => {
            let assets_path = plugin_root.join(assets_root.clone());
            println!("host assets from path {}", assets_path.to_str().unwrap());
            let assets_dir = ServeDir::new(assets_path.clone())
                .not_found_service(ServeFile::new(assets_path.join("index.html")));
            Router::new()
                .nest_service("/", assets_dir.clone())
                .fallback_service(assets_dir)
        }
    };

    let mut state = ServerState::new();

    let wasm_root = platx_config.wasm_root.clone();
    let wasm_router = match url::Url::parse(&wasm_root) {
        Ok(wasm_proxy_root) => {
            println!("proxy wasm service to url: {}", wasm_proxy_root.to_string());
            Router::new().route(
                "/invoke/:ty",
                post(
                    |State(state): State<Arc<Mutex<ServerState>>>, req: Request| async {
                        reverse_proxy_handler(state, req, wasm_proxy_root).await
                    },
                ),
            )
        }
        Err(_) => {
            println!(
                "host wasm service from path {}",
                plugin_root
                    .clone()
                    .join(platx_config.wasm_root.clone())
                    .to_str()
                    .unwrap()
            );
            state.with_wasm_context(plugin_root, platx_config)?;
            Router::new().route("/invoke/:ty", post(invoke_handler))
        }
    };

    let router = Router::new()
        .merge(assets_router)
        .merge(wasm_router)
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

async fn reverse_proxy_handler(
    state: Arc<Mutex<ServerState>>,
    req: Request,
    proxy_root: Url,
) -> axum::response::Response {
    let target_url = proxy_root.join(req.uri().to_string().as_str()).unwrap();

    println!("request redirect to {}", target_url);

    let client = state.lock().unwrap().reqwest_client.clone();

    let request_builder = client.request(req.method().clone(), target_url);
    let reqwest_response = client
        .execute(request_builder.build().unwrap())
        .await
        .expect("execute request failed");

    let mut response_builder =
        axum::response::Response::builder().status(reqwest_response.status());
    *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    response_builder
        .body(axum::body::Body::from_stream(
            reqwest_response.bytes_stream(),
        ))
        .unwrap()
}

struct ServerState {
    wasm_context: Option<WasmContext>,
    reqwest_client: reqwest::Client,
}

struct WasmContext {
    engine: Engine,
    linker: Linker<StoreState>,
    component: Component,
    plugin_root: PathBuf,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            wasm_context: None,
            reqwest_client: reqwest::Client::new(),
        }
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
