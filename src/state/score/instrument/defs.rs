use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Expression {
    Natural,
    Pizzicato,
    Spiccato,
    Staccato,
    Tremolo,
}

#[derive(Serialize)]
pub struct StaveDef {
    pub lines: u8,
    pub clef_pitch: u8, // these are the default clef for the instrument track
    pub clef_offset: u8,
}

impl StaveDef {
    pub fn new(lines: u8, clef_pitch: u8, clef_offset: u8) -> Self {
        Self {
            lines,
            clef_pitch,
            clef_offset: clef_offset,
        }
    }
}

#[derive(Serialize)]
pub struct InstrumentDef {
    pub id: &'static str,
    pub path: Vec<&'static str>,
    pub long_name: &'static str,
    pub short_name: &'static str,
    pub staves: Vec<StaveDef>,
    pub patches: HashMap<Expression, &'static str>,
}

lazy_static! {
    pub static ref INSTRUMENT_DEFS: Vec<InstrumentDef> = {
        vec![
            InstrumentDef {
                id: "keyboards.pianoforte",
                path: vec!["Keyboards", "Piano"],
                long_name: "Piano",
                short_name: "pno",
                staves: vec![StaveDef::new(5, 67, 6), StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/piano/natural.json"
                },
            },
            InstrumentDef {
                id: "strings.violin",
                path: vec!["Strings", "Violin"],
                long_name: "Violin",
                short_name: "Vln",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/violin/natural.json",
                    Expression::Pizzicato => "/patches/violin/pizzicato.json",
                    Expression::Spiccato => "/patches/violin/spiccato.json",
                    Expression::Tremolo => "/patches/violin/tremolo.json"
                },
            },
            InstrumentDef {
                id: "strings.viola",
                path: vec!["Strings", "Viola"],
                long_name: "Viola",
                short_name: "Vla",
                staves: vec![StaveDef::new(5, 60, 4)],
                patches: hashmap! {
                    Expression::Natural => "/patches/viola/natural.json",
                    Expression::Pizzicato => "/patches/viola/pizzicato.json",
                    Expression::Staccato => "/patches/viola/staccato.json"
                },
            },
            InstrumentDef {
                id: "strings.violoncello",
                path: vec!["Strings", "Violoncello"],
                long_name: "Violoncello",
                short_name: "Vc",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/violoncello/natural.json",
                    Expression::Pizzicato => "/patches/violoncello/pizzicato.json",
                    Expression::Staccato => "/patches/violoncello/staccato.json"
                },
            },
            InstrumentDef {
                id: "woodwinds.clarinet.a",
                path: vec!["Woodwinds", "Clarinet", "A"],
                long_name: "Clarinet in A",
                short_name: "Cl",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.clarinet.b-flat",
                path: vec!["Woodwinds", "Clarinet", "B Flat"],
                long_name: "Clarinet in B${flat}",
                short_name: "Cl",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
            },
        ]
    };
}

pub fn get_def(id: &str) -> Option<&InstrumentDef> {
    INSTRUMENT_DEFS.iter().find(|&def| def.id == id)
}

/// Get patches for a given id
#[wasm_bindgen]
pub fn get_patches(id: &str) -> JsValue {
    let def = match get_def(id) {
        Some(def) => def,
        None => return JsValue::UNDEFINED,
    };

    JsValue::from_serde(&def.patches).unwrap()
}
