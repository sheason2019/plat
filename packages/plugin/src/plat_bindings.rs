use std::fs;

use channel::ChannelHandler;
use lock::LockHandler;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::server::wasi::PlatServer;

wasmtime::component::bindgen!({
  path: "wit",
  world: "plat-world",
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

impl lock::Host for Component {
    fn create_lock_handler(
        &mut self,
        name: wasmtime::component::__internal::String,
    ) -> wasmtime::Result<Result<LockHandler, wasmtime::component::__internal::String>> {
        todo!()
    }

    fn drop_lock_handler(
        &mut self,
        handler: LockHandler,
    ) -> wasmtime::Result<Result<(), wasmtime::component::__internal::String>> {
        todo!()
    }

    fn lock(
        &mut self,
        handler: LockHandler,
    ) -> wasmtime::Result<Result<(), wasmtime::component::__internal::String>> {
        todo!()
    }

    fn unlock(
        &mut self,
        handler: LockHandler,
    ) -> wasmtime::Result<Result<(), wasmtime::component::__internal::String>> {
        todo!()
    }
}

impl channel::Host for Component {
    fn create_channel_handler(
        &mut self,
        name: wasmtime::component::__internal::String,
    ) -> wasmtime::Result<Result<ChannelHandler, String>> {
        todo!()
    }

    fn drop_channel_handler(
        &mut self,
        handler: ChannelHandler,
    ) -> wasmtime::Result<Result<(), String>> {
        todo!()
    }

    fn send(
        &mut self,
        handler: ChannelHandler,
        message: wasmtime::component::__internal::String,
    ) -> wasmtime::Result<Result<(), String>> {
        todo!()
    }

    fn recv(&mut self, handler: ChannelHandler) -> wasmtime::Result<Result<(), String>> {
        todo!()
    }
}

impl task::Host for Component {
    fn spawn(
        &mut self,
        payload: wasmtime::component::__internal::String,
    ) -> wasmtime::Result<Result<(), String>> {
        todo!()
    }
}

impl plat::Host for Component {
    fn sig(&mut self, source: Vec<u8>) -> wasmtime::Result<Result<Vec<u8>, String>> {
        todo!()
    }

    fn verify(&mut self, source: Vec<u8>, sig: Vec<u8>) -> wasmtime::Result<Result<bool, String>> {
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
