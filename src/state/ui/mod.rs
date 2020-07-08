pub mod play;
pub mod setup;

use crate::state::ui::play::Play;
use crate::state::ui::setup::Setup;
use crate::state::Engine;
use crate::utils::duration::NoteDuration;
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
pub struct Ui {
    pub view: View,
    pub snap: NoteDuration,
    pub flow_key: String,
    pub setup: Setup,
    pub play: Play,
}

impl Ui {
    pub fn new(flow_key: String) -> Ui {
        Ui {
            view: View::Setup,
            snap: NoteDuration::Sixteenth,
            flow_key: flow_key,
            setup: Setup::new(),
            play: Play::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_view(&mut self, value: View) {
        self.state.ui.view = value;
        self.emit();
    }
    pub fn set_snap(&mut self, value: NoteDuration) {
        self.state.ui.snap = value;
        self.emit();
    }
    pub fn set_flow_key(&mut self, value: &str) {
        self.state.ui.flow_key = String::from(value);
        self.emit();
    }
}
