use anyhow::Error;
use wasmtime::{
    Config, Engine, Store,
    component::{Component, Linker, bindgen},
};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../extension/wit",
});

// bindgen!({
//     path: "../facade/wit",
// });

/// The above will generate bindings for Extension
/// but I am not sure how I could also generate bindings for the facade component?

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Config
    let mut cfg = Config::new();
    cfg.async_support(true);

    // Engine
    let ngn = Engine::new(&cfg)?;

    // Context and Store
    let ctx = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();

    let mut store = Store::new(
        &ngn,
        State {
            ctx,
            tbl: ResourceTable::new(),
        },
    );

    // Linker
    let mut lnk = Linker::new(&ngn);
    wasmtime_wasi::add_to_linker_async(&mut lnk)?;

    // Facade
    let _fac = Component::from_file(&ngn, "target/wasm32-wasip2/debug/facade.wasm")?;

    // NOTE: I want to make sure the facade is linked so that the extension can call it
    // NOTE: but it seems you can only specify one world in the `bindgen!(...)` call above (line 8)

    // Extension
    let ext = Component::from_file(&ngn, "target/wasm32-wasip2/debug/extension.wasm")?;
    let ext = Extension::instantiate(&mut store, &ext, &lnk)?;

    // NOTE: I want to invoke `init` on the `extension` component
    ext.local_build_init().call_init(&mut store)?;

    Ok(())
}

struct State {
    ctx: WasiCtx,
    tbl: ResourceTable,
}

impl WasiView for State {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl IoView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.tbl
    }
}
