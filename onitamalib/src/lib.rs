extern crate console_error_panic_hook;
pub use messages::*;
pub use models::*;

mod cards;
mod board;
mod models;

mod agents;
pub use agents::*;

#[cfg(test)]
mod tests;

pub mod messages;

cfg_if::cfg_if! {
    if #[cfg(feature = "web")] {
        extern crate wasm_bindgen;
        use std::panic;
        use wasm_bindgen::prelude::*;

        pub use gamemodes::*;
        mod gamemodes;

        #[wasm_bindgen(start)]
        pub fn init(){
            wasm_logger::init(wasm_logger::Config::default());
            panic::set_hook(Box::new(console_error_panic_hook::hook));
        }
    }
}
