use crate::state::Engine;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Playback {
    metronome: bool,
}

impl Playback {
    pub fn new() -> Playback {
        Playback { metronome: false }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_metronome(&mut self, value: bool) {
        self.state.playback.metronome = value;
        self.emit();
    }
}
