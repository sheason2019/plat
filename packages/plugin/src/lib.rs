use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::{Method, Response};
use models::RegistedPlugin;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use wasi::PlatServer;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;

mod plat_bindings;
mod wasi;

pub struct PluginService {
    pub registed_plugin: models::RegistedPlugin,

    stop_server_sender: Sender<()>,
    service_handler: JoinHandle<()>,
}

impl PluginService {
    pub async fn new(
        plugin_config_path: PathBuf,
        daemon_address: String,
        regist_address: Option<String>,
        port: u16,
    ) -> anyhow::Result<Self> {
        if !plugin_config_path.is_absolute() {
            return Err(anyhow!(
                "plugin_config_path 必须为绝对路径，但它的值为：{}",
                plugin_config_path.to_str().unwrap()
            ));
        }

        let plugin_config = models::PluginConfig::from_file(plugin_config_path.clone())?;
        let plugin_config_directory = plugin_config_path.parent().unwrap().to_path_buf();

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let component = Component::from_file(
            &engine,
            plugin_config_directory.join(plugin_config.wasm_root.clone()),
        )?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)?;
        plat_bindings::lock::add_to_linker(&mut linker, |state: &mut plat_bindings::Component| {
            state
        })?;
        plat_bindings::task::add_to_linker(&mut linker, |state: &mut plat_bindings::Component| {
            state
        })?;
        plat_bindings::channel::add_to_linker(
            &mut linker,
            |state: &mut plat_bindings::Component| state,
        )?;

        let pre = plat_bindings::PlatWorldPre::new(linker.instantiate_pre(&component)?)?;
        let plat_server = Arc::new(PlatServer {
            pre,
            plugin_config,
            plugin_config_directory,
            daemon_address,
            lock_map: Arc::new(Mutex::new(HashMap::new())),
        });

        // plugin init
        {
            let mut store = Store::new(
                plat_server.pre.engine(),
                plat_bindings::Component::new(&plat_server),
            );
            let world = plat_server.pre.instantiate_async(&mut store).await?;
            world.lifecycle().call_before_start(&mut store).await?;
        }

        // plugin server
        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let plugin_address = format!("http://{}", listener.local_addr().unwrap());

        let handler = tokio::task::spawn({
            let plat_server = plat_server.clone();
            async move {
                loop {
                    let (client, _addr) = listener
                        .accept()
                        .await
                        .expect("plugin server accept failed");
                    let plat_server = plat_server.clone();
                    tokio::task::spawn(async move {
                        if let Err(_e) = http1::Builder::new()
                            .serve_connection(
                                TokioIo::new(client),
                                hyper::service::service_fn(move |req| {
                                    let plat_server = plat_server.clone();
                                    async move {
                                        let method = req.method();
                                        let uri = req.uri();
                                        let path = uri.path();

                                        match (method, path) {
                                            (&Method::GET, "/plugin.json") => {
                                                send_plugin_json(req, &plat_server.plugin_config)
                                            }
                                            (_method, _uri) => {
                                                plat_server.handle_request(req).await
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
                config: plat_server.plugin_config.clone(),
            },
            stop_server_sender: tx.clone(),
            service_handler,
        };

        // plugin regist
        let url = url::Url::parse(&plat_server.daemon_address)
            .context(format!(
                "parse daemon address {} failed",
                &plat_server.daemon_address
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

    pub fn addr(&self) -> &String {
        &self.registed_plugin.addr
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
    plugin_config: &models::PluginConfig,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    let plugin_json = plugin_config.to_json_string()?.as_bytes().to_vec();

    let body = Full::new(plugin_json.into())
        .map_err(|never| match never {})
        .boxed();
    let mut res = Response::new(body);
    res.headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());

    Ok(res)
}
