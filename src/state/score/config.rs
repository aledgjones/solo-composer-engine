use crate::state::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum InstrumentAutoCountStyle {
    Arabic,
    Roman,
}

#[derive(Serialize)]
pub struct AutoCountStyle {
    solo: InstrumentAutoCountStyle,
    section: InstrumentAutoCountStyle,
}

impl AutoCountStyle {
    pub fn new() -> AutoCountStyle {
        AutoCountStyle {
            solo: InstrumentAutoCountStyle::Roman,
            section: InstrumentAutoCountStyle::Roman,
        }
    }
}

#[derive(Serialize)]
pub struct Config {
    auto_count_style: AutoCountStyle,
}

impl Config {
    pub fn new() -> Config {
        Config {
            auto_count_style: AutoCountStyle::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_auto_count_style_solo(&mut self, value: InstrumentAutoCountStyle) {
        self.state.score.config.auto_count_style.solo = value;
        self.update();
        self.emit();
    }
    pub fn set_auto_count_style_section(&mut self, value: InstrumentAutoCountStyle) {
        self.state.score.config.auto_count_style.section = value;
        self.update();
        self.emit();
    }
}
