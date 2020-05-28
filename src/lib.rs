mod utils;

#[macro_use]
extern crate serde_derive;

use console_web::log;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct ScoreMeta {
    pub title: String,
    pub composer: String,
    pub created: i32,
}

#[wasm_bindgen]
pub fn run(mm: f32, score: &JsValue, flow_key: &str) {
    let meta: ScoreMeta = score.into_serde().unwrap();
    let title = meta.title;
    log!(mm, title, flow_key);
}
