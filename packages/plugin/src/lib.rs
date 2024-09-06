use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Context};
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::{Method, Response};
use models::RegistedPlugin;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use wasi::{PlatClientState, PlatServer};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;

mod wasi;

pub struct PluginService {
    registed_plugin: models::RegistedPlugin,

    stop_server_sender: Sender<()>,
    service_handler: JoinHandle<()>,
}

impl PluginService {
    pub async fn new(
        plugin_config: models::PluginConfig,
        plugin_root: PathBuf,
        daemon_address: String,
        regist_address: Option<String>,
        port: u16,
    ) -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let component =
            Component::from_file(&engine, plugin_root.join(plugin_config.wasm_root.clone()))?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)?;
        let pre = ProxyPre::new(linker.instantiate_pre(&component)?)?;
        let server = Arc::new(PlatServer { pre });

        // plugin init
        {
            let mut store = Store::new(
                server.pre.engine(),
                PlatClientState::new(plugin_root.clone(), daemon_address.clone()),
            );
            let instance = linker
                .instantiate_async(&mut store, &component)
                .await
                .expect("get plugin instance failed");
            let init_func = instance.get_typed_func::<(), ()>(&mut store, "on-init");
            if init_func.is_ok() {
                init_func
                    .unwrap()
                    .call_async(&mut store, ())
                    .await
                    .expect("plugin init func");
            }
        }

        // plugin server
        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let plugin_address = format!("http://{}", listener.local_addr().unwrap());

        let handler = tokio::task::spawn({
            let daemon_address = daemon_address.clone();
            let plugin_root = plugin_root.clone();
            async move {
                loop {
                    let (client, _addr) = listener
                        .accept()
                        .await
                        .expect("plugin server accept failed");
                    let server = server.clone();
                    let plugin_root = plugin_root.clone();
                    let plugin_json_path = plugin_root.clone().join("plugin.json");
                    let daemon_address = daemon_address.clone();
                    tokio::task::spawn(async move {
                        if let Err(_e) = http1::Builder::new()
                            .serve_connection(
                                TokioIo::new(client),
                                hyper::service::service_fn(move |req| {
                                    let server = server.clone();
                                    let plugin_root = plugin_root.clone();
                                    let plugin_json_path = plugin_json_path.clone();
                                    let daemon_address = daemon_address.clone();
                                    async move {
                                        let method = req.method();
                                        let uri = req.uri();
                                        let path = uri.path();

                                        match (method, path) {
                                            (&Method::GET, "/plugin.json") => {
                                                send_plugin_json(req, plugin_json_path)
                                            }
                                            (_method, _uri) => {
                                                server
                                                    .handle_request(
                                                        req,
                                                        plugin_root.clone(),
                                                        daemon_address.clone(),
                                                    )
                                                    .await
                                            }
                                        }
                                    }
                                }),
                            )
                            .await
                        {}
                    });
                }
            }
        });

        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        let service_handler = tokio::task::spawn(async move {
            rx.recv().await.unwrap();
            handler.abort();
        });

        let service = PluginService {
            registed_plugin: RegistedPlugin {
                addr: plugin_address,
                config: plugin_config,
            },
            stop_server_sender: tx.clone(),
            service_handler,
        };

        // plugin regist
        let url = url::Url::parse(&daemon_address)
            .context(format!(
                "parse daemon address {} failed",
                daemon_address.clone()
            ))?
            .join("plugin")?;
        let mut data = HashMap::new();
        match regist_address {
            Some(value) => {
                data.insert("addr", value.clone());
            }
            None => {
                data.insert("addr", service.registed_plugin.addr.clone());
            }
        }

        let client = reqwest::Client::new();
        let response = match client.post(url).json(&data).send().await {
            Err(e) => {
                service.stop().await;
                return Err(anyhow!("send regist plugin request failed: {}", e));
            }
            Ok(response) => response,
        };
        if !response.status().is_success() {
            service.stop().await;
            return Err(anyhow!("regist plugin failed: {}", response.text().await?));
        }

        Ok(service)
    }

    pub async fn stop(&self) {
        self.stop_server_sender.send(()).await.unwrap();
    }

    pub async fn wait(self) {
        self.service_handler.await.unwrap();
    }
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
