use wasmtime::component::bindgen;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::{WasiCtx, WasiView};

bindgen!({world: "plat", path: "src/core/wit/world.wit", async: true});

pub struct StoreState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl StoreState {
    pub fn new() -> Self {
        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdio();

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
