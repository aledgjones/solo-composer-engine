use crate::utils::shortid;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum NoteLength {
    Crotchet,
    DottedQuaver,
    Quaver,
    DottedSemiQuaver,
    SemiQuaver,
}

impl NoteLength {
    pub fn to_value(&self) -> f32 {
        match self {
            NoteLength::Crotchet => 1.0,
            NoteLength::DottedQuaver => 0.75,
            NoteLength::Quaver => 0.5,
            NoteLength::DottedSemiQuaver => 0.325,
            NoteLength::SemiQuaver => 0.25,
        }
    }
}

#[derive(Serialize)]
pub struct Flow {
    pub key: String,
    pub title: String,
    pub players: HashSet<String>, // purely for inclusion lookup -- order comes from score.players.order
    pub tick_length: NoteLength, // the smallest note length in the score (max: smallest time sig denominator)
    pub length: f32,             // number of crotchet beats in the flow
}

#[derive(Serialize)]
pub struct Flows {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Flow>,
}

impl Flows {
    pub fn new() -> Flows {
        let flow = Flow {
            key: shortid(),
            title: String::from("Untitled Flow"),
            players: HashSet::new(),
            tick_length: NoteLength::SemiQuaver,
            length: 1.0,
        };

        let order = vec![flow.key.clone()];

        let mut by_key = HashMap::new();
        by_key.insert(flow.key.clone(), flow);

        Flows { order, by_key }
    }
}
