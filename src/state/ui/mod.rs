use crate::state::Engine;
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
}

impl Ui {
    pub fn new() -> Ui {
        Ui { view: View::Setup }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_view(&mut self, value: View) {
        self.state.ui.view = value;
        self.emit();
    }
}
