use crate::state::Engine;
use js_sys::Date;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Meta {
    pub title: String,
    pub composer: String,
    pub created: f64,
}

impl Meta {
    pub fn new() -> Meta {
        Meta {
            title: String::from("Untitled Score"),
            composer: String::from("Unknown Composer"),
            created: Date::now(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_title(&mut self, value: String) {
        self.state.score.meta.title = value;
        self.emit();
    }
    pub fn set_composer(&mut self, value: String) {
        self.state.score.meta.composer = value;
        self.emit();
    }
}
