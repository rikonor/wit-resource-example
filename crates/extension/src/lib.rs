#[allow(warnings)]
mod bindings;

use bindings::{
    exports::local::build::{init, types},
    // local::build::registry::register_provider,
};

struct Component;

impl init::Guest for Component {
    fn init() -> () {
        println!("[extension] init called");
        // let b = bindings::exports::local::build::types::Builder::new(Tmp);
        // let b = unsafe { bindings::local::build::types::Builder::from_handle(b.handle()) };

        // register_provider(
        //     "my-canister-type", // canister_type
        //     b,                  // canister_builder
        // )
        // .expect("failed to register canister builder");
    }
}

struct Tmp;

impl types::GuestBuilder for Tmp {
    fn build_canister(&self, canister_dir: String) -> Result<(), String> {
        println!("[extension] build_canister called {canister_dir}");
        Ok(())
    }
}

impl types::Guest for Component {
    type Builder = Tmp;
}

bindings::export!(Component with_types_in bindings);
