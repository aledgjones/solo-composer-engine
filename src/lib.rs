mod state;
mod utils;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_repr;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
}
