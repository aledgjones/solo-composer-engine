mod render;
mod utils;

#[macro_use]
extern crate serde_derive;

use render::{Instruction, Instructions};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use render::text::{TextAlign, TextJustify, TextStyles};

#[derive(Serialize, Deserialize)]
pub struct ScoreMeta {
    pub title: String,
    pub composer: String,
    pub created: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Score {
    meta: ScoreMeta,
}

#[wasm_bindgen]
pub fn run(mm: f32, score: &JsValue, flow_key: String) -> JsValue {
    utils::set_panic_hook();
    let meta: ScoreMeta = score.into_serde().unwrap();
    let title = meta.title;
    let mut instructions = Instructions {
        space: 10,
        height: 100,
        width: 100,
        entries: Vec::new(),
    };
    let text = Instruction::Text {
        key: String::from("a"),
        identifier: String::from("text"),
        value: title,
        styles: TextStyles {
            color: String::from("#000000"),
            font: String::from("Roboto"),
            size: 6,
            align: TextAlign::Bottom,
            justify: TextJustify::Start,
        },
        x: 10,
        y: 10,
    };
    instructions.entries.push(text);
    JsValue::from_serde(&instructions).unwrap()
}
