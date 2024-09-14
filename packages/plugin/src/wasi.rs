use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use tokio::sync::Mutex;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::WasiHttpView;

use crate::plat_bindings;

pub struct PlatServer {
    pub pre: plat_bindings::PlatWorldPre<plat_bindings::Component>,
    pub plugin_config: models::PluginConfig,
    pub plugin_config_directory: PathBuf,
    pub daemon_address: String,

    pub lock_map: Arc<Mutex<HashMap<String, Arc<Mutex<()>>>>>,
}

impl PlatServer {
    pub fn new(plugin_config_path: PathBuf, daemon_address: String) -> anyhow::Result<Self> {
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
        Ok(PlatServer {
            pre,
            plugin_config,
            plugin_config_directory,
            daemon_address,
            lock_map: Arc::new(Mutex::new(HashMap::new())),
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
}
