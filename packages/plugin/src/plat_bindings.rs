use std::fs;

use wasmtime::component::ResourceTable;
use wasmtime_wasi::{async_trait, DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::server::wasi::PlatServer;

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
                .envs(&[
                    ("daemon_address", &plat_server.daemon_address),
                    ("daemon_public_key", &plat_server.daemon_public_key),
                ])
                .preopened_dir(storage_path, "/storage", DirPerms::all(), FilePerms::all())
                .unwrap()
                .preopened_dir(assets_path, "/assets", DirPerms::all(), FilePerms::all())
                .unwrap()
                .build(),
            http: WasiHttpCtx::new(),
        }
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
