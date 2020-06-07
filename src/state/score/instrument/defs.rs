use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum Expression {
    Natural,
    Pizzicato,
    Spiccato,
    Tremolo,
}

impl Expression {
    pub fn to_string(&self) -> &'static str {
        match self {
            Expression::Natural => "natural",
            Expression::Pizzicato => "pizzicato",
            Expression::Spiccato => "spiccato",
            Expression::Tremolo => "tremolo",
        }
    }
    pub fn from_string(value: &'static str) -> Expression {
        if value == "pizzicato" {
            Expression::Natural
        } else if value == "spiccato" {
            Expression::Spiccato
        } else if value == "tremolo" {
            Expression::Tremolo
        } else {
            Expression::Natural
        }
    }
}

#[derive(Serialize)]
pub struct StaveDef {
    lines: u8,
    clef: &'static str,
    offset: u8,
}

#[derive(Serialize)]
pub struct InstrumentDef {
    pub id: &'static str,
    pub path: Vec<&'static str>,
    pub long_name: &'static str,
    pub short_name: &'static str,
    pub staves: Vec<StaveDef>,
    pub patches: HashMap<&'static str, &'static str>,
}

lazy_static! {
    pub static ref INSTRUMENT_DEFS: HashMap<String, InstrumentDef> = {
        hashmap! {
            String::from("strings.violin") => InstrumentDef {
                id: "strings.violin",
                path: vec!["Strings", "Violin"],
                long_name: "Violin",
                short_name: "Vln.",
                staves: vec![StaveDef {
                    lines: 5,
                    clef: "G4",
                    offset: 6,
                }],
                patches: hashmap! {
                    Expression::Natural.to_string() => "/patches/violin/natural.json",
                    Expression::Pizzicato.to_string() => "/patches/violin/pizzicato.json",
                    Expression::Spiccato.to_string() => "/patches/violin/spiccato.json",
                    Expression::Tremolo.to_string() => "/patches/violin/tremolo.json"
                },
            }
        }
    };
}
