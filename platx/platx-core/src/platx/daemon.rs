use std::{
    collections::HashMap,
    future::Future,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Buf, Bytes},
    server::conn::http1,
    Method, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use wasmtime_wasi_http::io::TokioIo;

pub struct PlatXDaemon {
    pub addr: String,
    pub plugin_map: Arc<Mutex<HashMap<String, RegistedPlugin>>>,
    context: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistedPlugin {
    pub addr: String,
    pub config: PlatXConfig,
}

impl PlatXDaemon {
    pub fn new() -> Self {
        PlatXDaemon {
            addr: String::new(),
            plugin_map: Arc::new(Mutex::new(HashMap::new())),
            context: String::new(),
        }
    }

    pub fn with_context(&mut self, context: String) {
        self.context = context;
    }

    pub async fn start_server(&mut self) -> anyhow::Result<impl Future> {
        let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        self.addr = format!("http://{}", tcp_listener.local_addr().unwrap().to_string());

        let context = self.context.clone();
        let plugin_map = self.plugin_map.clone();
        let deamon_handler = tokio::task::spawn(async move {
            loop {
                let context = context.clone();
                let plugin_map = plugin_map.clone();
                let (client, _addr) = tcp_listener
                    .accept()
                    .await
                    .expect("accept tcp listener failed");
                tokio::spawn(async move {
                    let context = context.clone();
                    let plugin_map = plugin_map.clone();
                    let _ = http1::Builder::new()
                        .keep_alive(true)
                        .preserve_header_case(true)
                        .title_case_headers(true)
                        .serve_connection(
                            TokioIo::new(client),
                            hyper::service::service_fn(|req| {
                                let context = context.clone();
                                let plugin_map = plugin_map.clone();
                                async move {
                                    match (req.method(), req.uri().path()) {
                                        (&Method::GET, "/plugin") => {
                                            get_plugins_handler(plugin_map.clone()).await
                                        }
                                        (&Method::POST, "/plugin") => {
                                            regist_handler(req, plugin_map.clone()).await
                                        }
                                        (&Method::GET, "/context") => {
                                            get_context_handler(context.clone()).await
                                        }
                                        (_method, _path) => notfound(),
                                    }
                                }
                            }),
                        )
                        .with_upgrades()
                        .await;
                });
            }
        });

        let plugin_map = self.plugin_map.clone();
        let health_handler = tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;

                let mut remove_key: Vec<String> = Vec::new();
                let keys: Vec<String> = plugin_map
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|i| i.0.clone())
                    .collect();
                for key in keys {
                    let registed_plugin_addr =
                        plugin_map.lock().unwrap().get(&key).unwrap().addr.clone();
                    let u = url::Url::parse(&registed_plugin_addr)
                        .unwrap()
                        .join("plugin.json")
                        .unwrap();
                    if reqwest::get(u).await.is_err() {
                        remove_key.push(key.clone());
                    }
                }

                for key in remove_key {
                    plugin_map.as_ref().lock().unwrap().remove(&key);
                }
            }
        });

        Ok(async move {
            health_handler.await.unwrap();
            deamon_handler.await.unwrap();
        })
    }

    pub fn serilize_plugins(&self) -> anyhow::Result<String> {
        let plugin_map = self.plugin_map.lock().unwrap();
        let json_string = serde_json::to_string(plugin_map.deref())?;
        Ok(json_string)
    }

    pub fn uninstall_plugin(&self, name: &str) -> anyhow::Result<()> {
        self.plugin_map.lock().unwrap().remove(name);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatXConfig {
    pub name: String,
    pub version: String,
    pub wasm_root: String,
    pub entries: Vec<PlatXEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatXEntry {
    pub label: String,
    pub icon: String,
    pub href: String,
    pub target: String,
}

impl PlatXConfig {
    pub fn from_path(dir_path: PathBuf) -> anyhow::Result<Self> {
        let plugin_file = dir_path.join("plugin.json");
        let plugin_bytes = std::fs::read(plugin_file)?;
        let config: PlatXConfig = serde_json::from_slice(&plugin_bytes)?;

        Ok(config)
    }
}

async fn regist_handler(
    req: hyper::Request<hyper::body::Incoming>,
    plugin_map: Arc<Mutex<HashMap<String, RegistedPlugin>>>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let whole_body = req.collect().await?.aggregate();
    let body: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    let addr = body
        .as_object()
        .expect("invalid input")
        .get("addr")
        .expect("parse addr faield")
        .as_str()
        .expect("parse addr as string failed");
    let target =
        url::Url::parse(addr).expect(format!("parse addr {} as url failed", &addr).as_ref());

    let config = reqwest::get(target.join("plugin.json").unwrap())
        .await
        .expect("request regist plugin failed")
        .json::<PlatXConfig>()
        .await
        .expect("json deserilize failed");
    println!("plugin {} registed", config.name);

    let registed_plugin = RegistedPlugin {
        addr: addr.to_string(),
        config,
    };

    plugin_map
        .lock()
        .unwrap()
        .insert(registed_plugin.config.name.clone(), registed_plugin);

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(
            Full::new("OK".into())
                .map_err(|never| match never {})
                .boxed(),
        )
        .unwrap();
    Ok(response)
}

async fn get_plugins_handler(
    plugin_map: Arc<Mutex<HashMap<String, RegistedPlugin>>>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let value = serde_json::to_string(plugin_map.lock().unwrap().deref())?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(
            Full::new(value.into())
                .map_err(|never| match never {})
                .boxed(),
        )
        .unwrap();
    Ok(response)
}

fn notfound() -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("".into()).map_err(|never| match never {}).boxed())
        .unwrap();
    Ok(response)
}

async fn get_context_handler(
    context: String,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(
            Full::new(context.into())
                .map_err(|never| match never {})
                .boxed(),
        )
        .unwrap();
    Ok(response)
}
