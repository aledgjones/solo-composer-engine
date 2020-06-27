use crate::state::Engine;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum View {
    File,
    Setup,
    Write,
    Engrave,
    Play,
    Print,
}

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum Tool {
    Select,
    Draw,
    Slice,
    Erase,
}

#[derive(Serialize)]
pub struct UiPlay {
    pub zoom: f32,
    pub keyboard: HashMap<String, u8>,
    pub tool: Tool,
}

impl UiPlay {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            keyboard: HashMap::new(),
            tool: Tool::Select,
        }
    }
}

#[derive(Serialize)]
pub struct Ui {
    view: View,
    expanded: HashMap<String, bool>, // expressed in js as object -- perfect for quick lookups
    play: UiPlay,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            view: View::Setup,
            expanded: HashMap::new(),
            play: UiPlay::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_view(&mut self, value: View) {
        self.state.ui.view = value;
        self.emit();
    }
    pub fn expand(&mut self, key: &str) {
        self.state.ui.expanded.insert(String::from(key), true);
        self.emit();
    }
    pub fn collapse(&mut self, key: &str) {
        self.state.ui.expanded.remove(key);
        self.emit();
    }
    pub fn set_play_keyboard(&mut self, key: &str, offset: u8) {
        self.state
            .ui
            .play
            .keyboard
            .insert(String::from(key), offset);
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
