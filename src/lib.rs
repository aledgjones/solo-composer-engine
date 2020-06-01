mod state;
mod utils;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
}
