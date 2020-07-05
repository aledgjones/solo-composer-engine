use crate::state::ui::Tool;
use crate::state::Engine;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Keyboard {
    base: u8,
    height: u8,
}

impl Keyboard {
    pub fn new(base: u8, height: u8) -> Self {
        Self { base, height }
    }
}

#[derive(Serialize)]
pub struct Play {
    pub zoom: f32,
    pub keyboard: HashMap<String, Keyboard>,
    pub expanded: HashMap<String, bool>,
    pub tool: Tool,
}

impl Play {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            keyboard: HashMap::new(),
            expanded: HashMap::new(),
            tool: Tool::Select,
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn play_expand(&mut self, key: &str) {
        self.state.ui.play.expanded.insert(String::from(key), true);
        self.emit();
    }
    pub fn play_collapse(&mut self, key: &str) {
        self.state.ui.play.expanded.remove(key);
        self.emit();
    }
    pub fn set_play_keyboard(&mut self, key: &str, base: u8, height: u8) {
        self.state
            .ui
            .play
            .keyboard
            .insert(String::from(key), Keyboard::new(base, height));
        self.emit();
    }
    pub fn set_play_tool(&mut self, value: Tool) {
        self.state.ui.play.tool = value;
        self.emit();
    }
    pub fn set_play_zoom(&mut self, value: f32) {
        self.state.ui.play.zoom = value;
        self.emit();
    }
}
