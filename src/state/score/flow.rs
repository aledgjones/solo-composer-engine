use crate::state::entries::time_signature::{TimeSignature, TimeSignatureDrawType};
use crate::state::score::instrument::defs::get_def;
use crate::state::score::stave::Stave;
use crate::state::score::track::Track;
use crate::state::Engine;
use crate::utils::shortid;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Flow {
    pub key: String,
    pub title: String,
    pub players: HashSet<String>, // purely for inclusion lookup -- order comes from score.players.order
    pub length: u32,              // number of subdivision ticks in the flow
    pub subdivisions: u8,         // how many times to subdevide the crotchet

    pub master: Track,
    pub staves: HashMap<String, Stave>,
    pub tracks: HashMap<String, Track>,
}

impl Flow {
    pub fn new() -> Flow {
        let mut flow = Flow {
            key: shortid(),
            title: String::from(""),
            players: HashSet::new(),
            length: 4,       // 1 crotchet beat
            subdivisions: 4, // auto it to semi-quavers

            master: Track::new(),
            staves: HashMap::new(),
            tracks: HashMap::new(),
        };

        flow.master.insert(TimeSignature::new(
            shortid(),
            0,
            0,
            4,
            TimeSignatureDrawType::Hidden,
            None,
        ));

        flow
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
            let def = match get_def(&instrument.id.as_str()) {
                Some(instrument_def) => instrument_def,
                None => return JsValue::UNDEFINED,
            };

            for (i, stave_key) in instrument.staves.iter().enumerate() {
                let track = Track::new();
                let mut stave = Stave::new(stave_key.clone(), &def.staves[i]);
                stave.tracks.push(track.key.clone());
                flow.tracks.insert(track.key.clone(), track);
                flow.staves.insert(stave.key.clone(), stave);
            }
        }

        self.state.score.flows.order.push(flow.key.clone());
        self.state.score.flows.by_key.insert(flow.key.clone(), flow);

        self.update();
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
        self.update();
        self.emit();
    }

    pub fn reorder_flow(&mut self, old_index: usize, new_index: usize) {
        let removed = self.state.score.flows.order.remove(old_index);
        self.state.score.flows.order.insert(new_index, removed);
        self.update();
        self.emit();
    }

    pub fn remove_flow(&mut self, flow_key: &str) {
        self.state.score.flows.order.retain(|e| e != flow_key);
        self.state.score.flows.by_key.remove(flow_key);
        self.update();
        self.emit();
    }

    /**
     * Assign a player to a flow
     */
    pub fn assign_player(&mut self, flow_key: &str, player_key: &str) {
        // add the player_key to the flow
        let flow = match self.state.score.flows.by_key.get_mut(flow_key) {
            Some(flow) => flow,
            None => return (),
        };
        flow.players.insert(String::from(player_key));

        // get all the insturments assigned to the player
        let instrument_keys = match self.state.score.players.by_key.get(player_key) {
            // we need to clone so instrument_keys isn't a ref to self and so we can use it later
            Some(player) => player.instruments.clone(),
            None => return (),
        };

        // add staves and tracks to this flow
        for instrument_key in instrument_keys {
            let instrument = match self.state.score.instruments.get(&instrument_key) {
                Some(instrument) => instrument,
                None => return (),
            };
            let def = match get_def(&instrument.id.as_str()) {
                Some(instrument_def) => instrument_def,
                None => return (),
            };

            for (i, stave_key) in instrument.staves.iter().enumerate() {
                let track = Track::new();
                let mut stave = Stave::new(stave_key.clone(), &def.staves[i]);
                stave.tracks.push(track.key.clone());
                flow.tracks.insert(track.key.clone(), track);
                flow.staves.insert(stave.key.clone(), stave);
            }
        }

        self.update();
        self.emit();
    }

    pub fn unassign_player(&mut self, flow_key: &str, player_key: &str) {
        // remove the player_key from the flow
        let flow = match self.state.score.flows.by_key.get_mut(flow_key) {
            Some(flow) => flow,
            None => return (),
        };
        flow.players.remove(player_key);

        // get all the insturments assigned to the player
        let instrument_keys = match self.state.score.players.by_key.get(player_key) {
            // we need to clone so instrument_keys isn't a ref to self and so we can use it later
            Some(player) => player.instruments.clone(),
            None => return (),
        };

        // delete staves and tracks in this flow
        for instrument_key in instrument_keys {
            let stave_keys = match self.state.score.instruments.get(&instrument_key) {
                Some(instrument) => &instrument.staves,
                None => return (),
            };
            for stave_key in stave_keys {
                let stave = match flow.staves.get(stave_key) {
                    Some(stave) => stave,
                    None => return (),
                };
                for track_key in &stave.tracks {
                    flow.tracks.remove(track_key);
                }
                flow.staves.remove(stave_key);
            }
        }

        self.update();
        self.emit();
    }

    pub fn ticks(&self, flow_key: &str, zoom: f32) -> JsValue {
        let crotchet_width = 72.0 * zoom;
        let mut ticks = TickList {
            list: Vec::new(),
            width: 0.0,
        };
        let flow = match self.state.score.flows.by_key.get(flow_key) {
            Some(flow) => flow,
            None => return JsValue::from_serde(&ticks).unwrap(), // return an empty tick list
        };
        let mut result: Option<&TimeSignature> = None;

        for tick in 0..flow.length {
            match flow.master.get_time_signature_at_tick(tick) {
                Some(time_signature) => {
                    result = Some(time_signature);
                }
                None => (),
            };

            let time_signature = match &result {
                Some(time_signature) => time_signature,
                None => return JsValue::from_serde(&ticks).unwrap(), // this will never happen so return early if it does
            };

            let ticks_per_crotchet = time_signature.ticks_per_beat_type(flow.subdivisions, 4);
            let tick_width = crotchet_width / ticks_per_crotchet as f32;

            ticks.push(Tick {
                x: ticks.width,
                width: tick_width,
                is_beat: time_signature.is_on_beat(tick, flow.subdivisions),
                is_first_beat: time_signature.is_on_first_beat(tick, flow.subdivisions),
                is_quaver_beat: time_signature.is_on_beat_type(tick, flow.subdivisions, 8),
                is_grouping_boundry: time_signature.is_on_grouping_boundry(tick, flow.subdivisions),
            })
        }

        JsValue::from_serde(&ticks).unwrap()
    }
}

#[derive(Serialize)]
struct TickList {
    list: Vec<Tick>,
    width: f32,
}

impl TickList {
    pub fn push(&mut self, tick: Tick) {
        self.width += tick.width;
        self.list.push(tick);
    }
}

#[derive(Serialize)]
struct Tick {
    x: f32,
    width: f32,
    is_beat: bool,
    is_first_beat: bool,
    is_quaver_beat: bool,
    is_grouping_boundry: bool,
}
