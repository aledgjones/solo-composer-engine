use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}

impl NoteDuration {
    pub fn to_int(&self) -> u8 {
        match self {
            NoteDuration::Whole => 1,
            NoteDuration::Half => 2,
            NoteDuration::Quarter => 4,
            NoteDuration::Eighth => 8,
            NoteDuration::Sixteenth => 16,
            NoteDuration::ThirtySecond => 32,
        }
    }

    pub fn to_ticks(&self, subdivisions: u8) -> u8 {
        let beat_type = self.to_int();
        (subdivisions as f32 / (beat_type as f32 / 4 as f32)) as u8
    }

    pub fn to_glyph(&self) -> &str {
        match self {
            NoteDuration::Whole => "\u{1D15D}",
            NoteDuration::Half => "\u{1D15E}",
            NoteDuration::Quarter => "\u{1D15F}",
            NoteDuration::Eighth => "\u{1D160}",
            NoteDuration::Sixteenth => "\u{1D161}",
            NoteDuration::ThirtySecond => "\u{1D162}",
        }
    }
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
