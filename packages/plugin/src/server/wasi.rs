use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, Context};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::WasiHttpView;

use crate::models::Plugin;
use crate::plat_bindings;

pub struct PlatServer {
    pub pre: plat_bindings::PlatWorldPre<plat_bindings::Component>,
    pub daemon_public_key: String,
    pub plugin_config: Plugin,
    pub plugin_config_directory: PathBuf,
    pub daemon_address: String,
}

impl PlatServer {
    pub fn new(plugin_config_path: PathBuf, daemon_address: String) -> anyhow::Result<Self> {
        if !plugin_config_path.is_absolute() {
            return Err(anyhow!(
                "plugin_config_path 必须为绝对路径，但它的值为：{}",
                plugin_config_path.to_str().unwrap()
            ));
        }

        let plugin_config_directory = plugin_config_path.parent().unwrap().to_path_buf();
        let plugin_config_bytes = fs::read(plugin_config_path).context("读取 plugin.json 失败")?;
        let plugin_config: Plugin =
            serde_json::from_slice(&plugin_config_bytes).context("序列化 plugin.json 失败")?;

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config).context("创建 WASI Engine 失败")?;

        let component = Component::from_file(
            &engine,
            plugin_config_directory.join(plugin_config.wasm_root.clone()),
        )
        .context("初始化 WASI Component 失败")?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).context("添加 Wasmtime WASI 链接失败")?;
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .context("添加 Wasmtime WASI HTTP 链接失败")?;
        plat_bindings::lock::add_to_linker(&mut linker, |state: &mut plat_bindings::Component| {
            state
        })
        .context("添加 Plat Lock 链接失败")?;
        plat_bindings::task::add_to_linker(&mut linker, |state: &mut plat_bindings::Component| {
            state
        })
        .context("添加 Plat Task 链接失败")?;
        plat_bindings::channel::add_to_linker(
            &mut linker,
            |state: &mut plat_bindings::Component| state,
        )
        .context("添加 Plat Channel 链接失败")?;

        let pre = plat_bindings::PlatWorldPre::new(
            linker
                .instantiate_pre(&component)
                .context("构建 instance_pre 失败")?,
        )
        .context("构建 plat_world_pre 失败")?;
        Ok(PlatServer {
            pre,
            plugin_config,
            plugin_config_directory,
            daemon_address,
            daemon_public_key: String::new(),
        })
    }

    pub async fn handle_request(
        &self,
        req: hyper::Request<hyper::body::Incoming>,
    ) -> Result<hyper::Response<HyperOutgoingBody>> {
        let mut store = Store::new(self.pre.engine(), plat_bindings::Component::new(&self));
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let req = store.data_mut().new_incoming_request(Scheme::Http, req)?;
        let out = store.data_mut().new_response_outparam(sender)?;
        let pre = self.pre.clone();

        let task = tokio::task::spawn(async move {
            let proxy = pre.instantiate(&mut store)?;

            if let Err(e) = proxy
                .wasi_http_incoming_handler()
                .call_handle(store, req, out)
            {
                return Err(e);
            }

            Ok(())
        });

        match receiver.await {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(e.into()),

            Err(_) => {
                let e = match task.await {
                    Ok(r) => r.unwrap_err(),
                    Err(e) => e.into(),
                };
                anyhow::bail!("guest never invoked `response-outparam::set` method: {e:?}")
            }
        }
    }

    pub async fn create_regist_plugin_handle(&mut self) -> anyhow::Result<Sender<()>> {
        let mut regist_plugin_address = Url::parse(&self.daemon_address)?.join("api/regist")?;
        match regist_plugin_address.scheme() {
            "http" => {
                regist_plugin_address.set_scheme("ws").unwrap();
            }
            "https" => {
                regist_plugin_address.set_scheme("wss").unwrap();
            }
            _ => {}
        };

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async(regist_plugin_address.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();

        match read.next().await.unwrap()? {
            Message::Text(daemon_public_key) => {
                self.daemon_public_key = daemon_public_key;
            }
            _ => return Err(anyhow!("注册 Plugin 失败")),
        }

        write
            .send(Message::text(
                serde_json::to_string(&self.plugin_config).unwrap(),
            ))
            .await?;

        let (tx, _rx) = tokio::sync::broadcast::channel::<()>(4);
        tokio::task::spawn({
            let tx = tx.clone();
            async move {
                let mut sub = tx.subscribe();
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(10)) => break,
                        _ = sub.recv() => break,
                        recv = read.next() => {
                            match recv {
                                Some(msg) => {
                                    match msg {
                                        Err(_) => break,
                                        Ok(msg) => {
                                            match msg {
                                                Message::Ping(inner) => {
                                                    write.send(Message::Pong(inner)).await.unwrap();
                                                },
                                                _ => (),
                                            }
                                        },
                                    }
                                },
                                None => break,
                            }
                        },
                    }
                }
                let _ = tx.send(());
            }
        });

        Ok(tx)
    }
}
