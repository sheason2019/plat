use std::path::PathBuf;
use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::{Method, Response};
use plugin_pre::PluginPre;
use tokio::sync::broadcast::Sender;
use wasi::PlatServer;
use wasmtime::{Result, Store};
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;

mod plat_bindings;
mod plugin_pre;
mod wasi;

pub struct PluginService {
    pub registed_plugin: models::RegistedPlugin,
    terminate: Sender<()>,
}

impl PluginService {
    pub async fn new(
        plugin_config_path: PathBuf,
        daemon_address: String,
        regist_address: Option<String>,
        port: u16,
    ) -> anyhow::Result<Self> {
        let plat_server = PlatServer::new(plugin_config_path, daemon_address.clone())?;
        let plat_server = Arc::new(plat_server);

        let plugin_pre = PluginPre::new(
            daemon_address,
            plat_server.plugin_config.clone(),
            regist_address,
            port,
        )
        .await?;
        let registed_plugin = plugin_pre.registed_plugin.clone();

        let terminate = plugin_pre.create_regist_plugin_handle().await?;

        // plugin init
        let init_handler = tokio::task::spawn({
            let plat_server = plat_server.clone();
            async move {
                let mut store = Store::new(
                    plat_server.pre.engine(),
                    plat_bindings::Component::new(&plat_server),
                );
                let world = plat_server.pre.instantiate_async(&mut store).await.unwrap();
                world.lifecycle().call_on_start(&mut store).await.unwrap();
            }
        });
        tokio::task::spawn({
            let terminate = terminate.clone();
            async move {
                let _ = terminate.subscribe().recv().await;
                init_handler.abort();
            }
        });

        tokio::task::spawn({
            let terminate = terminate.clone();
            let plat_server = plat_server.clone();
            async move {
                let mut sub = terminate.subscribe();
                loop {
                    let (client, _addr) = tokio::select! {
                        val = plugin_pre.tcp_listener.accept() => val,
                        _ = sub.recv() => break,
                    }
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

        Ok(PluginService {
            registed_plugin,
            terminate,
        })
    }

    pub fn addr(&self) -> &String {
        &self.registed_plugin.addr
    }

    pub async fn stop(&self) {
        let _ = self.terminate.send(());
    }

    pub async fn wait(self) {
        let _ = self.terminate.subscribe().recv().await;
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
