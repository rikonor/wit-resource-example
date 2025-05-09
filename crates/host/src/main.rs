use anyhow::{Context, Error};
use wasmtime::{
    Config, Engine, Store,
    component::{Component, Linker, Val, bindgen},
};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

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

    // Extension -> Facade (Dynamic Link for `register`)
    let inst = lnk
        .instance("local:build/register")
        .context("failed to get instance")?;

    println!("1");

    // inst.func_new_async("register", move |mut store, params, results| {
    //     let fname = fname.clone();
    //     let fref = Arc::clone(&fref);

    //     Box::new(async move {
    //         let f = {
    //             let g = fref.lock().unwrap();
    //             *g.as_ref()
    //                 .ok_or_else(|| DynamicLinkingError::UnresolvedReference(fname))?
    //         };

    //         f.call_async(&mut store, params, results)
    //             .await
    //             .context("call failed")?;

    //         f.post_return_async(&mut store)
    //             .await
    //             .context("post-return failed")?;

    //         Ok(())
    //     })
    // })?;

    // Extension (load)
    let ext_cmpnt = Component::from_file(&ngn, "target/wasm32-wasip2/debug/extension.wasm")?;

    // Facade (load)
    let fac_cmpnt = Component::from_file(&ngn, "target/wasm32-wasip2/debug/facade.wasm")?;

    // Extension (link)
    let ext_inst = lnk
        .instantiate_async(&mut store, &ext_cmpnt)
        .await
        .context("failed to instantiate extension")?;

    // Facade (link)
    let fac_inst = lnk
        .instantiate_async(&mut store, &fac_cmpnt)
        .await
        .context("failed to instantiate facade")?;

    // Extension (init)
    let e = ext_inst
        .get_export(&mut store, None, "local:build/init")
        .context("failed to find export")?;

    let e = ext_inst
        .get_export(&mut store, Some(&e), "init")
        .context("failed to find export")?;

    let f = ext_inst
        .get_func(&mut store, &e)
        .context("failed to find function")?;

    f.call_async(&mut store, &[], &mut [])
        .await
        .context("failed to call function")?;

    f.post_return_async(&mut store)
        .await
        .context("failed to post-return")?;

    // Facade (init)
    let e = fac_inst
        .get_export(&mut store, None, "local:build/init")
        .context("failed to find export")?;

    let e = fac_inst
        .get_export(&mut store, Some(&e), "init")
        .context("failed to find export")?;

    let f = fac_inst
        .get_func(&mut store, &e)
        .context("failed to find function")?;

    f.call_async(&mut store, &[], &mut [])
        .await
        .context("failed to call function")?;

    f.post_return_async(&mut store)
        .await
        .context("failed to post-return")?;

    // At this point the extension should have called the facade to register itself

    // Facade (build)
    let e = fac_inst
        .get_export(&mut store, None, "local:build/build")
        .context("failed to find export")?;

    let e = fac_inst
        .get_export(&mut store, Some(&e), "build")
        .context("failed to find export")?;

    let f = fac_inst
        .get_func(&mut store, &e)
        .context("failed to find function")?;

    let mut results = vec![Val::Option(None)];
    f.call_async(&mut store, &[], &mut results)
        .await
        .context("failed to call function")?;

    f.post_return_async(&mut store)
        .await
        .context("failed to post-return")?;

    println!("{results:?}");

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
