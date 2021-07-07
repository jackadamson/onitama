#![feature(array_map)]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use std::panic;

use wasm_bindgen::prelude::*;

pub use gamemodes::*;
pub use messages::*;
pub use models::*;

mod cards;
mod board;
mod models;

#[cfg(feature = "agent")]
mod agents;
#[cfg(feature = "agent")]
pub use agents::*;

#[cfg(test)]
mod tests;

mod gamemodes;
pub mod messages;

#[wasm_bindgen(start)]
pub fn init(){
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
