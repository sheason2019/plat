use std::path::PathBuf;

use wasmtime::component::ResourceTable;
use wasmtime::{Result, Store};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

pub struct PlatServer {
    pub pre: ProxyPre<PlatClientState>,
}

impl PlatServer {
    pub async fn handle_request(
        &self,
        req: hyper::Request<hyper::body::Incoming>,
        plugin_dir: PathBuf,
        daemon_address: String,
    ) -> Result<hyper::Response<HyperOutgoingBody>> {
        let mut store = Store::new(
            self.pre.engine(),
            PlatClientState::new(plugin_dir, daemon_address),
        );
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

pub struct PlatClientState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl PlatClientState {
    pub fn new(plugin_root: PathBuf, daemon_address: String) -> Self {
        PlatClientState {
            table: ResourceTable::new(),
            wasi: WasiCtxBuilder::new()
                .inherit_stdio()
                .envs(&[("daemon_address", &daemon_address)])
                .preopened_dir(
                    plugin_root.join("storage"),
                    "/storage",
                    DirPerms::all(),
                    FilePerms::all(),
                )
                .unwrap()
                .build(),
            http: WasiHttpCtx::new(),
        }
    }
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
