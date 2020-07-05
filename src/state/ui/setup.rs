use crate::state::Engine;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Setup {
    pub expanded: HashMap<String, bool>,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            expanded: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn setup_expand(&mut self, key: &str) {
        self.state.ui.setup.expanded.insert(String::from(key), true);
        self.emit();
    }
    pub fn setup_collapse(&mut self, key: &str) {
        self.state.ui.setup.expanded.remove(key);
        self.emit();
    }
}
