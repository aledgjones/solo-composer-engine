use crate::state::Engine;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum View {
    Setup,
    Write,
    Engrave,
    Play,
    Print,
}

#[derive(Serialize)]
pub struct Ui {
    view: View,
    expanded: HashMap<String, bool>, // expressed in js as object -- perfect for quick lookups
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            view: View::Setup,
            expanded: HashMap::new(),
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
}
