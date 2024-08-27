use std::fs;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::{Method, Response};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use super::PlatXConfig;

pub struct ServerHandler {
    pub handler: JoinHandle<()>,
    pub addr: String,
}

pub async fn start_server(
    plugin_root: PathBuf,
    platx_config: PlatXConfig,
) -> anyhow::Result<ServerHandler> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, plugin_root.join(platx_config.wasm_root))?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)?;
    let pre = ProxyPre::new(linker.instantiate_pre(&component)?)?;

    let server = Arc::new(PlatServer { pre });

    let listener = TcpListener::bind("127.0.0.1:0").await?;

    let init_handler = tokio::task::spawn({
        let server = server.clone();
        async move {
            let mut store = Store::new(
                server.pre.engine(),
                PlatClientState {
                    table: ResourceTable::new(),
                    wasi: WasiCtxBuilder::new().inherit_stdio().build(),
                    http: WasiHttpCtx::new(),
                },
            );
            let instance = linker
                .instantiate_async(&mut store, &component)
                .await
                .expect("get plugin instance failed");
            let init_func = match instance.get_typed_func::<(), ()>(&mut store, "on-init") {
                Ok(value) => value,
                Err(e) => {
                    println!("已跳过 Plugin 初始化逻辑，原因：{}", e.to_string());
                    return;
                }
            };

            init_func
                .call_async(&mut store, ())
                .await
                .expect("plugin init func");
        }
    });

    let addr = format!("http://{}", listener.local_addr().unwrap());

    let server_handler = tokio::task::spawn(async move {
        loop {
            let (client, _addr) = listener
                .accept()
                .await
                .expect("plugin server accept failed");
            let server = server.clone();
            let plugin_root = plugin_root.clone();
            let plugin_json_path = plugin_root.clone().join("plugin.json");
            tokio::task::spawn(async move {
                if let Err(_e) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(
                        TokioIo::new(client),
                        hyper::service::service_fn(move |req| {
                            let server = server.clone();
                            let plugin_root = plugin_root.clone();
                            let plugin_json_path = plugin_json_path.clone();
                            async move {
                                match (req.method(), req.uri().path()) {
                                    (&Method::GET, "/plugin.json") => {
                                        send_plugin_json(req, plugin_json_path)
                                    }
                                    (_method, _uri) => {
                                        server.handle_request(req, plugin_root.clone()).await
                                    }
                                }
                            }
                        }),
                    )
                    .await
                {}
            });
        }
    });

    let handler = tokio::task::spawn(async move {
        let _ = init_handler.await;
        let _ = server_handler.await;
    });
    Ok(ServerHandler {
        handler,
        addr: addr.clone(),
    })
}

fn send_plugin_json(
    _req: hyper::Request<hyper::body::Incoming>,
    plugin_json_path: impl AsRef<Path>,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    let plugin_json = fs::read(plugin_json_path)?;

    let body = Full::new(plugin_json.into())
        .map_err(|never| match never {})
        .boxed();
    let mut res = Response::new(body);
    res.headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());

    Ok(res)
}

struct PlatServer {
    pre: ProxyPre<PlatClientState>,
}

impl PlatServer {
    async fn handle_request(
        &self,
        req: hyper::Request<hyper::body::Incoming>,
        plugin_dir: PathBuf,
    ) -> Result<hyper::Response<HyperOutgoingBody>> {
        // Create per-http-request state within a `Store` and prepare the
        // initial resources  passed to the `handle` function.
        let mut store = Store::new(
            self.pre.engine(),
            PlatClientState {
                table: ResourceTable::new(),
                wasi: WasiCtxBuilder::new()
                    .inherit_stdio()
                    .preopened_dir(
                        plugin_dir.join("storage"),
                        "/storage",
                        DirPerms::all(),
                        FilePerms::all(),
                    )
                    .unwrap()
                    .build(),
                http: WasiHttpCtx::new(),
            },
        );
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let req = store.data_mut().new_incoming_request(Scheme::Http, req)?;
        let out = store.data_mut().new_response_outparam(sender)?;
        let pre = self.pre.clone();

        // Run the http request itself in a separate task so the task can
        // optionally continue to execute beyond after the initial
        // headers/response code are sent.
        let task = tokio::task::spawn(async move {
            let proxy = pre.instantiate_async(&mut store).await?;

            if let Err(e) = proxy
                .wasi_http_incoming_handler()
                .call_handle(store, req, out)
                .await
            {
                return Err(e);
            }

            Ok(())
        });

        match receiver.await {
            // If the client calls `response-outparam::set` then one of these
            // methods will be called.
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(e.into()),

            // Otherwise the `sender` will get dropped along with the `Store`
            // meaning that the oneshot will get disconnected and here we can
            // inspect the `task` result to see what happened
            Err(_) => {
                let e = match task.await {
                    Ok(r) => r.unwrap_err(),
                    Err(e) => e.into(),
                };
                anyhow::bail!("guest never invoked `response-outparam::set` method: {e:?}")
            }
        }
    }
}

struct PlatClientState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for PlatClientState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for PlatClientState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
