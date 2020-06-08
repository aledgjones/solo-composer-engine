use crate::log;
use crate::state::score::instrument::defs::INSTRUMENT_DEFS;
use crate::state::score::stave::Stave;
use crate::state::score::track::Track;
use crate::state::Engine;
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

    pub staves: HashMap<String, Stave>,
    pub tracks: HashMap<String, Track>,
}

impl Flow {
    pub fn new() -> Flow {
        Flow {
            key: shortid(),
            title: Flow::default_title(),
            players: HashSet::new(),
            tick_length: NoteLength::SemiQuaver,
            length: 1.0,
            staves: HashMap::new(),
            tracks: HashMap::new(),
        }
    }
    pub fn default_title() -> String {
        String::from("Untitled Flow")
    }
}

#[derive(Serialize)]
pub struct Flows {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Flow>,
}

impl Flows {
    pub fn new() -> Flows {
        let flow = Flow::new();
        let order = vec![flow.key.clone()];

        let mut by_key = HashMap::new();
        by_key.insert(flow.key.clone(), flow);

        Flows { order, by_key }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_flow(&mut self) -> JsValue {
        let mut flow = Flow::new();
        let flow_key = flow.key.clone(); // return value

        // add all the players into the new flow
        for player_key in &self.state.score.players.order {
            flow.players.insert(player_key.clone());
        }

        // add stave / tracks for each instrument in the score
        for (_instrument_key, instrument) in &self.state.score.instruments {
            let instrument_def = match INSTRUMENT_DEFS.get(&instrument.id) {
                Some(instrument_def) => instrument_def,
                None => return JsValue::UNDEFINED,
            };

            for (i, stave_key) in instrument.staves.iter().enumerate() {
                let track = Track::new();
                let mut stave = Stave::new(stave_key.clone(), &instrument_def.staves[i]);
                stave.tracks.push(track.key.clone());
                flow.tracks.insert(track.key.clone(), track);
                flow.staves.insert(stave.key.clone(), stave);
            }
        }

        self.state.score.flows.order.push(flow.key.clone());
        self.state.score.flows.by_key.insert(flow.key.clone(), flow);

        self.emit();

        JsValue::from_str(&flow_key)
    }

    pub fn rename_flow(&mut self, flow_key: &str, name: &str) {
        match self.state.score.flows.by_key.get_mut(flow_key) {
            Some(flow) => {
                flow.title = String::from(name);
            }
            None => return (),
        };
        self.emit();
    }

    pub fn reorder_flow(&mut self, old_index: usize, new_index: usize) {
        let removed = self.state.score.flows.order.remove(old_index);
        self.state.score.flows.order.insert(new_index, removed);
        self.emit();
    }

    pub fn remove_flow(&mut self, flow_key: &str) {
        self.state.score.flows.order.retain(|e| e != flow_key);
        self.state.score.flows.by_key.remove(flow_key);
        self.emit();
    }
}
