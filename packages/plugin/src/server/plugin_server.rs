use std::path::PathBuf;
use std::sync::Arc;

use crate::models::Plugin;
use crate::server::wasi::PlatServer;
use anyhow::Context;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{Method, Request, Response};
use tokio::net::TcpListener;
use tokio::sync::broadcast::Sender;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin};
use wasmtime::{Result, Store};
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;

use super::Options;

pub struct PluginServer {
    terminate: Sender<()>,
    plat_server: Arc<PlatServer>,
    pub path: PathBuf,
    pub server_address: String,
    pub regist_address: String,
}

impl PluginServer {
    pub async fn new(plugin_path: PathBuf, options: Options) -> anyhow::Result<Self> {
        let plugin_dir = plugin_path.parent().unwrap().to_path_buf();
        let daemon_address = options.daemon_address;

        let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", options.port))
            .await
            .context("监听端口失败")?;

        let mut plat_server =
            PlatServer::new(plugin_dir.join("plugin.json"), daemon_address.clone())
                .context("创建 PlatServer 失败")?;
        let server_address = format!("http://{}", tcp_listener.local_addr()?);
        let regist_address = match options.regist_address.as_ref() {
            Some(address) => address.clone(),
            None => server_address.clone(),
        };
        plat_server
            .plugin_config
            .address
            .replace(regist_address.clone());

        let terminate = plat_server
            .create_regist_plugin_handle()
            .await
            .context("注册 Plugin 失败")?;

        let plat_server = Arc::new(plat_server);

        // plugin init
        let init_handler = tokio::task::spawn({
            let plat_server = plat_server.clone();
            async move {
                let mut store = Store::new(
                    plat_server.pre.engine(),
                    crate::plat_bindings::Component::new(&plat_server),
                );
                let world = plat_server.pre.instantiate_async(&mut store).await.unwrap();
                world
                    .lifecycle()
                    .call_on_start(&mut store)
                    .await
                    .expect("调用 onStart 生命周期失败");
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
                        val = tcp_listener.accept() => val,
                        _ = sub.recv() => break,
                    }
                    .expect("plugin server accept failed");

                    let plat_server = plat_server.clone();
                    tokio::task::spawn(async move {
                        let svc = tower::service_fn(move |req: Request<Incoming>| {
                            let plat_server = plat_server.clone();
                            async move {
                                let method = req.method();
                                let uri = req.uri();
                                let path = uri.path();

                                match (method, path) {
                                    (&Method::GET, "/plugin.json") => {
                                        send_plugin_json(req, &plat_server.plugin_config)
                                    }
                                    (_method, _uri) => plat_server.handle_request(req).await,
                                }
                            }
                        });
                        let svc = tower::ServiceBuilder::new()
                            .layer(
                                tower_http::cors::CorsLayer::new()
                                    .allow_methods(AllowMethods::mirror_request())
                                    .allow_origin(AllowOrigin::mirror_request())
                                    .allow_credentials(true)
                                    .allow_headers(AllowHeaders::mirror_request()),
                            )
                            .service(svc);
                        let svc = hyper_util::service::TowerToHyperService::new(svc);

                        if let Err(e) = http1::Builder::new()
                            .serve_connection(TokioIo::new(client), svc)
                            .await
                        {
                            println!("handle request error: {:?}", e);
                        }
                    });
                }
            }
        });

        Ok(PluginServer {
            path: plugin_dir,
            plat_server,
            server_address,
            regist_address,
            terminate,
        })
    }

    pub fn plugin(&self) -> &Plugin {
        &self.plat_server.plugin_config
    }

    pub async fn stop(&self) {
        let _ = self.terminate.send(());
    }

    pub async fn wait(self) {
        let _ = self.terminate.subscribe().recv().await;
    }
}

fn send_plugin_json(
    _req: Request<Incoming>,
    plugin_config: &crate::models::Plugin,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    let plugin_json = serde_json::to_string(&plugin_config)?.as_bytes().to_vec();

    let body = Full::new(plugin_json.into())
        .map_err(|never| match never {})
        .boxed();
    let mut res = Response::new(body);
    res.headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());

    Ok(res)
}
