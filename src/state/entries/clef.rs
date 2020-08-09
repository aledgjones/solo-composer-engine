use crate::state::entries::Entry;
use crate::utils::measurements::{BoundingBox, Padding, Spaces};
use crate::utils::pitch::{Accidental, Pitch};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u8)]
pub enum ClefDrawType {
    Hidden,
    G,
    F,
    C,
    Percussion,
}

#[derive(Serialize, Deserialize)]
pub struct Clef {
    pub key: String,
    pub tick: u32,
    pub draw_as: ClefDrawType,
    pub pitch: Pitch, // the pitch that the clef sits on
    pub offset: i8,   // visual offset from top stave line
}

impl Clef {
    pub fn new(key: String, tick: u32, pitch: u8, offset: i8, draw_as: ClefDrawType) -> Entry {
        Entry::Clef(Self {
            key,
            tick,
            draw_as,
            pitch: Pitch::new(pitch, Accidental::Natural),
            offset,
        })
    }

    fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: Spaces(2.8),
            height: Spaces(4.0),
            padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
        }
    }
}
