use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use wasmtime::component::ResourceTable;
use wasmtime::{Result, Store};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

pub struct PlatServer {
    pub pre: ProxyPre<PlatClientState>,
    pub plugin_config: models::PluginConfig,
    pub plugin_config_directory: PathBuf,
    pub daemon_address: String,
    pub lock_id_map: Arc<Mutex<HashMap<String, Arc<Mutex<()>>>>>,
}

impl PlatServer {
    pub async fn handle_request(
        &self,
        req: hyper::Request<hyper::body::Incoming>,
    ) -> Result<hyper::Response<HyperOutgoingBody>> {
        // 由于 WASI 缺乏文件锁，这里使用 LockID 在请求层实现锁机制
        let req_lock = Arc::new(Mutex::new(()));

        let lock_id = req.headers().get("lock-id");
        if lock_id.is_some() {
            let lock_id_str = lock_id.unwrap().to_str()?;
            let exist_lock = {
                let lock_map = self.lock_id_map.lock().await;
                let lock = lock_map.get(lock_id_str);
                if lock.is_some() {
                    Some(lock.unwrap().clone())
                } else {
                    None
                }
            };

            if exist_lock.is_some() {
                let _ = exist_lock.unwrap().lock().await;
            }

            self.lock_id_map
                .lock()
                .await
                .insert(lock_id_str.to_string(), req_lock.clone());
        }

        let req_lock = req_lock.lock().await;

        let mut store = Store::new(self.pre.engine(), PlatClientState::new(&self));
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

        let res = match receiver.await {
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
        };

        drop(req_lock);
        
        res
    }
}

pub struct PlatClientState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl PlatClientState {
    pub fn new(plat_server: &PlatServer) -> Self {
        let storage_path = plat_server
            .plugin_config_directory
            .join(&plat_server.plugin_config.storage_root);
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path).unwrap();
        }

        let assets_path = plat_server
            .plugin_config_directory
            .join(&plat_server.plugin_config.assets_root);
        if !assets_path.exists() {
            fs::create_dir_all(&assets_path).unwrap();
        }

        PlatClientState {
            table: ResourceTable::new(),
            wasi: WasiCtxBuilder::new()
                .inherit_stdio()
                .envs(&[("daemon_address", &plat_server.daemon_address)])
                .preopened_dir(storage_path, "/storage", DirPerms::all(), FilePerms::all())
                .unwrap()
                .preopened_dir(assets_path, "/assets", DirPerms::all(), FilePerms::all())
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
