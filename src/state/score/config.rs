use crate::state::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum AutoCountStyle {
    Arabic,
    Roman,
}

#[derive(Serialize)]
pub struct AutoCount {
    pub solo: AutoCountStyle,
    pub section: AutoCountStyle,
}

impl AutoCount {
    pub fn new() -> AutoCount {
        AutoCount {
            solo: AutoCountStyle::Roman,
            section: AutoCountStyle::Roman,
        }
    }
}

#[derive(Serialize)]
pub struct Config {
    pub auto_count: AutoCount,
}

impl Config {
    pub fn new() -> Config {
        Config {
            auto_count: AutoCount::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_auto_count_style_solo(&mut self, value: AutoCountStyle) {
        self.state.config.auto_count.solo = value;
        self.update();
        self.emit();
    }
    pub fn set_auto_count_style_section(&mut self, value: AutoCountStyle) {
        self.state.config.auto_count.section = value;
        self.update();
        self.emit();
    }
}
