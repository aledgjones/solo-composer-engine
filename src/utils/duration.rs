use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum NoteDuration {
    Double,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}

#[derive(Serialize)]
pub struct Duration {
    // number of ticks in the duration
    pub int: u32,
    // TODO: this should include a way of specifying how the duration should be written
    // ie. if the user wants it written differently to the parsing algorithm
}

impl Duration {
    pub fn new(int: u32) -> Self {
        Self { int }
    }
}
