use std::{collections::HashMap, fs, sync::Arc};

use anyhow::anyhow;
use tokio::sync::{mpsc::Sender, Mutex};
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{async_trait, DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::wasi::PlatServer;

wasmtime::component::bindgen!({
  path: "wit",
  world: "plat-world",
  async: true,
  trappable_imports: true,
  require_store_data_send: true,
  with: {
      "wasi:io": wasmtime_wasi::bindings::io,

      "wasi:http/types/outgoing-body": wasmtime_wasi_http::body::HostOutgoingBody,
      "wasi:http/types/future-incoming-response": wasmtime_wasi_http::types::HostFutureIncomingResponse,
      "wasi:http/types/outgoing-response": wasmtime_wasi_http::types::HostOutgoingResponse,
      "wasi:http/types/future-trailers": wasmtime_wasi_http::body::HostFutureTrailers,
      "wasi:http/types/incoming-body": wasmtime_wasi_http::body::HostIncomingBody,
      "wasi:http/types/incoming-response": wasmtime_wasi_http::types::HostIncomingResponse,
      "wasi:http/types/response-outparam": wasmtime_wasi_http::types::HostResponseOutparam,
      "wasi:http/types/outgoing-request": wasmtime_wasi_http::types::HostOutgoingRequest,
      "wasi:http/types/incoming-request": wasmtime_wasi_http::types::HostIncomingRequest,
      "wasi:http/types/fields": wasmtime_wasi_http::types::HostFields,
      "wasi:http/types/request-options": wasmtime_wasi_http::types::HostRequestOptions,
  },
  trappable_error_type: {
      "wasi:http/types/error-code" => wasmtime_wasi_http::HttpError,
  },
});

pub struct Component {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
    pub lock_handler: Arc<Mutex<LockHandler>>,
}

impl Component {
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

        Component {
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
            lock_handler: Arc::new(Mutex::new(LockHandler {
                lock_map: plat_server.lock_map.clone(),
                current_lock: HashMap::new(),
            })),
        }
    }
}

impl Drop for Component {
    fn drop(&mut self) {
        todo!()
    }
}

#[async_trait]
impl lock::Host for Component {
    async fn create_lock_handler(&mut self, name: String) -> wasmtime::Result<lock::LockHandler> {
        todo!()
    }

    async fn drop_lock_handler(&mut self, handler: lock::LockHandler) -> wasmtime::Result<()> {
        todo!()
    }

    async fn lock(&mut self, handler: lock::LockHandler) -> wasmtime::Result<()> {
        // self.lock_handler.lock().await.lock(id).await
        todo!()
    }

    async fn unlock(&mut self, handler: lock::LockHandler) -> wasmtime::Result<()> {
        // self.lock_handler.lock().await.unlock(id).await
        todo!()
    }
}

#[async_trait]
impl channel::Host for Component {
    async fn create_channel_handler(
        &mut self,
        name: String,
    ) -> wasmtime::Result<channel::ChannelHandler> {
        todo!()
    }

    async fn drop_channel_handler(
        &mut self,
        handler: channel::ChannelHandler,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    async fn send(
        &mut self,
        handler: channel::ChannelHandler,
        message: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    async fn recv(&mut self, handler: channel::ChannelHandler) -> wasmtime::Result<String> {
        todo!()
    }
}

#[async_trait]
impl task::Host for Component {
    async fn spawn(&mut self, payload: String) -> wasmtime::Result<()> {
        todo!()
    }
}

impl WasiView for Component {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for Component {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub struct LockHandler {
    lock_map: Arc<Mutex<HashMap<String, Arc<Mutex<()>>>>>,
    current_lock: HashMap<String, Sender<()>>,
}

impl LockHandler {
    async fn lock(&mut self, id: String) -> wasmtime::Result<()> {
        // 如果当前 Component 重复申请同一 ID 的锁则会造成死锁，此时抛出异常拒绝申请锁
        {
            let lock = self.current_lock.get(&id);
            if lock.is_some() {
                return Err(anyhow!("在同一个上下文内重复申请了相同的锁"));
            }
        }

        // 循环直到获取锁
        let current_lock = Arc::new(Mutex::new(()));
        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        tokio::task::spawn({
            let current_lock = current_lock.clone();
            async move {
                let _ = current_lock.lock().await;
                rx.recv().await;
            }
        });
        self.current_lock.insert(id.clone(), tx);

        loop {
            let mut lock_map = self.lock_map.lock().await;
            let lock = match lock_map.get(&id) {
                None => None,
                Some(value) => Some(value.clone()),
            };

            match lock {
                Some(value) => {
                    drop(lock_map);
                    let _ = value.lock().await;
                }
                None => {
                    lock_map.insert(id.clone(), current_lock.clone());
                    drop(lock_map);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn unlock(&mut self, id: String) -> wasmtime::Result<()> {
        match self.current_lock.remove(&id) {
            None => return Ok(()),
            Some(sender) => {
                sender.send(()).await?;
            }
        }

        self.lock_map.lock().await.remove(&id);

        Ok(())
    }

    pub async fn clean_lock(&mut self) -> anyhow::Result<()> {
        let keys: Vec<String> = self.current_lock.keys().map(|i| i.clone()).collect();
        for key in keys {
            self.unlock(key).await?;
        }

        Ok(())
    }
}
