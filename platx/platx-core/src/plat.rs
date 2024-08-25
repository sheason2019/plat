use std::path::PathBuf;

use wasmtime::component::bindgen;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::DirPerms;
use wasmtime_wasi::FilePerms;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::{WasiCtx, WasiView};

bindgen!({world: "plat", path: "src/wit/world.wit"});

pub struct StoreState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl StoreState {
    pub fn new(plugin_dir: PathBuf) -> Self {
        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdio();
        builder
            .preopened_dir(
                plugin_dir.join("static"),
                "/static",
                DirPerms::all(),
                FilePerms::all(),
            )
            .expect("preopen dir failed");

        StoreState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        }
    }
}

impl WasiView for StoreState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl PlatImports for StoreState {
    fn emit(&mut self, _ty: String, _payload: String) -> String {
        todo!()
    }
}
