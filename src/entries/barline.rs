use crate::utils::generic::{BoundingBox, Padding, Spaces};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum BarlineType {
    Double,
    EndRepeat,
    EndStartRepeat,
    Final,
    Normal,
    StartRepeat,
}

#[derive(Debug, Serialize)]
pub struct Barline {
    key: String,
    tick: f32,
    barline_type: BarlineType,
}

impl Barline {
    fn r#box(&self) -> BoundingBox {
        match self.barline_type {
            BarlineType::Double => BoundingBox {
                width: Spaces(0.5),
                height: Spaces(4.0),
                padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
            },
            BarlineType::EndRepeat => BoundingBox {
                width: Spaces(2.0),
                height: Spaces(4.0),
                padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
            },
            BarlineType::EndStartRepeat => BoundingBox {
                width: Spaces(2.0),
                height: Spaces(4.0),
                padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
            },
            BarlineType::Final => BoundingBox {
                width: Spaces(1.0),
                height: Spaces(4.0),
                padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
            },
            BarlineType::Normal => BoundingBox {
                width: Spaces(0.0),
                height: Spaces(4.0),
                padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
            },
            BarlineType::StartRepeat => BoundingBox {
                width: Spaces(2.0),
                height: Spaces(4.0),
                padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
            },
        }
    }
}
