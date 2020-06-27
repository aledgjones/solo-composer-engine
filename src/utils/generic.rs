use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct MM(pub f32);

#[derive(Serialize)]
pub struct Spaces(pub f32);

#[derive(Serialize)]
pub struct BoundingBox {
    pub width: Spaces,
    pub height: Spaces,
    pub padding: Padding<Spaces>,
}

#[derive(Serialize)]
pub struct Padding<T>(pub T, pub T, pub T, pub T);

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum Justify {
    Start,
    Middle,
    End,
}

impl Justify {
    fn to_string(&self) -> String {
        match self {
            Justify::Start => String::from("flex-start"),
            Justify::Middle => String::from("center"),
            Justify::End => String::from("flex-end"),
        }
    }
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum Align {
    Top,
    Middle,
    Bottom,
}

impl Align {
    fn to_string(&self) -> String {
        match self {
            Align::Top => String::from("flex-start"),
            Align::Middle => String::from("center"),
            Align::Bottom => String::from("flex-end"),
        }
    }
}

#[derive(Serialize)]
pub struct Font {
    pub size: Spaces,
    pub font: String,
    pub align: Justify,
    pub padding: Padding<Spaces>,
}
