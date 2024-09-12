use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;
use wasmtime::{Result, Store};
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
            let lock_handler = store.data().lock_handler.clone();

            if let Err(e) = proxy
                .wasi_http_incoming_handler()
                .call_handle(store, req, out)
                .await
            {
                return Err(e);
            }

            lock_handler.lock().await.clean_lock().await?;

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
