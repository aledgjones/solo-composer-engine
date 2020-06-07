use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum BarlineType {
    Normal,
    Double,
    Final,
    StartRepeat,
    EndRepeat,
    EndStartRepeat,
}
