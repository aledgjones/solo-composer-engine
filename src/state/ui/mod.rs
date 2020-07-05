pub mod play;
pub mod setup;

use crate::state::ui::play::Play;
use crate::state::ui::setup::Setup;
use crate::state::Engine;
use crate::utils::duration::NoteDuration;
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
    view: View,
    snap: NoteDuration,
    setup: Setup,
    play: Play,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            view: View::Setup,
            snap: NoteDuration::Sixteenth,
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
}
