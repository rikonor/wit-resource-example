#[allow(warnings)]
mod bindings;

use bindings::exports::local::build::registry;

struct Component;

impl registry::Guest for Component {
    fn register_provider(
        canister_type: String,
        canister_builder: registry::Builder,
    ) -> Result<(), String> {
        println!("[facade] register_provider called {canister_type}");
        canister_builder.build_canister("my-canister-dir")
    }
}

bindings::export!(Component with_types_in bindings);
