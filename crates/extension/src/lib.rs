use bindings::{exports::local::build::init, local::build::register::register};

#[allow(warnings)]
mod bindings;

struct Component;

// use bindings::{
//     exports::local::build::init,
//     // exports::local::build::{init, types},
//     local::build::registry::register_provider,
// };

impl init::Guest for Component {
    fn init() -> () {
        println!("[extension] init called");
        if let Err(err) = register("some-builder") {
            println!("failed to call register: {err}");
        };
    }
}

// impl init::Guest for Component {
//     fn init() -> () {
//         println!("[extension] init called");
//         // let b = bindings::exports::local::build::types::Builder::new(Tmp);
//         // let b = unsafe { bindings::local::build::types::Builder::from_handle(b.handle()) };

//         register_provider("my-canister-type").expect("failed to register canister builder");

//         // register_provider(
//         //     "my-canister-type", // canister_type
//         //     b,                  // canister_builder
//         // )
//         // .expect("failed to register canister builder");
//     }
// }

// struct Tmp;

// impl types::GuestBuilder for Tmp {
//     fn build_canister(&self, canister_dir: String) -> Result<(), String> {
//         println!("[extension] build_canister called {canister_dir}");
//         Ok(())
//     }
// }

// impl types::Guest for Component {
//     type Builder = Tmp;
// }

bindings::export!(Component with_types_in bindings);
