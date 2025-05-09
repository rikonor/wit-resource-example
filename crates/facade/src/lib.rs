use bindings::exports::local::build::{build, init, register};

#[allow(warnings)]
mod bindings;

struct Component;

// use bindings::exports::local::build::registry;

impl init::Guest for Component {
    fn init() -> () {
        println!("[facade] init called");
    }
}

impl build::Guest for Component {
    fn build() -> Result<(), String> {
        println!("[facade] build called");
        Ok(())
    }
}

impl register::Guest for Component {
    fn register(name: String) -> Result<(), String> {
        println!("[facade] register called: {name}");
        Ok(())
    }
}

// impl registry::Guest for Component {
//     fn register_provider(
//         canister_type: String,
//         _canister_builder: registry::Builder,
//     ) -> Result<(), String> {
//         println!("[facade] register_provider called {canister_type}");
//         // canister_builder.build_canister("my-canister-dir")
//         Ok(())
//     }
// }

// impl registry::Guest for Component {
//     fn register_provider(
//         canister_type: String,
//     ) -> Result<(), String> {
//         println!("[facade] register_provider called {canister_type}");
//         Ok(())
//     }
// }

bindings::export!(Component with_types_in bindings);
