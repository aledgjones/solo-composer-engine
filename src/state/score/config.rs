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
pub struct AutoCountConfig {
    pub active: bool,
    pub style: InstrumentAutoCountStyle,
}

#[derive(Serialize)]
pub struct AutoCount {
    pub solo: AutoCountConfig,
    pub section: AutoCountConfig,
}

impl AutoCount {
    pub fn new() -> AutoCount {
        AutoCount {
            solo: AutoCountConfig {
                active: true,
                style: InstrumentAutoCountStyle::Roman,
            },
            section: AutoCountConfig {
                active: true,
                style: InstrumentAutoCountStyle::Roman,
            },
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
    pub fn set_auto_count_style_solo(&mut self, value: InstrumentAutoCountStyle) {
        self.state.score.config.auto_count.solo.style = value;
        self.update();
        self.emit();
    }
    pub fn set_auto_count_style_section(&mut self, value: InstrumentAutoCountStyle) {
        self.state.score.config.auto_count.section.style = value;
        self.update();
        self.emit();
    }
    pub fn set_auto_count_active_solo(&mut self, value: bool) {
        self.state.score.config.auto_count.solo.active = value;
        self.update();
        self.emit();
    }
    pub fn set_auto_count_active_section(&mut self, value: bool) {
        self.state.score.config.auto_count.section.active = value;
        self.update();
        self.emit();
    }
}
