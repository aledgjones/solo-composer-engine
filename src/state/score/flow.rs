use crate::state::entries::clef::Clef;
use crate::state::entries::time_signature::{TimeSignature, TimeSignatureDrawType};
use crate::state::score::instrument::defs::get_def;
use crate::state::score::instrument::Instrument;
use crate::state::score::stave::Stave;
use crate::state::score::track::Track;
use crate::state::Engine;
use crate::utils::duration::NoteDuration;
use crate::utils::shortid;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Tick {
    pub tick: u32,
    pub x: f32,
    pub width: f32,
    pub is_beat: bool,
    pub is_first_beat: bool,
    pub is_quaver_beat: bool,
    pub is_grouping_boundry: bool,
}

#[derive(Serialize)]
pub struct TickList {
    pub list: Vec<Tick>,
    pub width: f32,
}

impl TickList {
    pub fn push(&mut self, tick: Tick) {
        self.width += tick.width;
        self.list.push(tick);
    }
}

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
            length: 16,       // 1 crotchet beat
            subdivisions: 16, // auto it to 32nd notes as this is the shortest snap

            master: Track::new(),
            staves: HashMap::new(),
            tracks: HashMap::new(),
        };

        flow.master.insert(TimeSignature::new(
            shortid(),
            0,
            0,
            NoteDuration::Quarter,
            TimeSignatureDrawType::Hidden, // a sort of fake time signature shown as hidden
            None,
        ));

        flow
    }

    pub fn add_instrument(&mut self, instrument: &Instrument) {
        let def = match get_def(&instrument.id.as_str()) {
            Some(instrument_def) => instrument_def,
            None => return (),
        };

        for (i, stave_key) in instrument.staves.iter().enumerate() {
            let track = Track::new();
            let clef = match def.staves.get(i) {
                Some(def) => Clef::new(shortid(), 0, def.clef_pitch, def.clef_offset),
                None => return (),
            };

            let mut stave = Stave::new(stave_key.clone(), &def.staves[i]);
            stave.master.insert(clef);
            stave.tracks.push(track.key.clone());
            self.tracks.insert(track.key.clone(), track);
            self.staves.insert(stave.key.clone(), stave);
        }
    }

    pub fn get_ticks(&self) -> TickList {
        let crotchet_width = 72.0;
        let mut ticks = TickList {
            list: Vec::new(),
            width: 0.0,
        };

        let mut result: Option<&TimeSignature> = None;

        for tick in 0..self.length {
            match self.master.get_time_signature_at_tick(tick) {
                Some(time_signature) => {
                    result = Some(time_signature);
                }
                None => (),
            };

            let time_signature = match result {
                Some(time_signature) => time_signature,
                None => return ticks, // this will never happen so return early if it does
            };

            let ticks_per_crotchet = NoteDuration::Quarter.to_ticks(self.subdivisions);
            let tick_width = crotchet_width / ticks_per_crotchet as f32;

            ticks.push(Tick {
                tick,
                x: ticks.width,
                width: tick_width,
                is_beat: time_signature.is_on_beat(tick, self.subdivisions),
                is_first_beat: time_signature.is_on_first_beat(tick, self.subdivisions),
                is_quaver_beat: time_signature.is_on_beat_type(
                    tick,
                    self.subdivisions,
                    &NoteDuration::Eighth,
                ),
                is_grouping_boundry: time_signature.is_on_grouping_boundry(tick, self.subdivisions),
            });
        }

        ticks
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
        let mut order = Vec::new();
        order.push(flow.key.clone());
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

        // add all the player keys into the new flow
        for player_key in &self.state.players.order {
            flow.players.insert(player_key.clone());
        }

        // add stave / tracks for each instrument in the score
        // we do this for every player so we can loop the instruments directly
        for (_instrument_key, instrument) in &self.state.instruments {
            flow.add_instrument(instrument);
        }

        self.state.flows.order.push(flow.key.clone());
        self.state.flows.by_key.insert(flow.key.clone(), flow);

        self.update();
        self.emit();

        JsValue::from_str(&flow_key)
    }

    pub fn rename_flow(&mut self, flow_key: &str, name: &str) {
        match self.state.flows.by_key.get_mut(flow_key) {
            Some(flow) => {
                flow.title = String::from(name);
            }
            None => return (),
        };
        self.update();
        self.emit();
    }

    pub fn set_flow_length(&mut self, flow_key: &str, length: u32) {
        match self.state.flows.by_key.get_mut(flow_key) {
            Some(flow) => {
                flow.length = length;
            }
            None => return (),
        };

        self.update();
        self.emit();
    }

    pub fn reorder_flow(&mut self, old_index: u8, new_index: u8) {
        let removed = self.state.flows.order.remove(old_index as usize);
        self.state.flows.order.insert(new_index as usize, removed);
        self.update();
        self.emit();
    }

    pub fn remove_flow(&mut self, flow_key: &str) {
        self.state.flows.order.retain(|e| e != flow_key);
        self.state.flows.by_key.remove(flow_key);
        self.update();
        self.emit();
    }

    /**
     * Assign a player to a flow
     */
    pub fn assign_player(&mut self, flow_key: &str, player_key: &str) {
        // add the player_key to the flow
        let flow = match self.state.flows.by_key.get_mut(flow_key) {
            Some(flow) => flow,
            None => return (),
        };

        flow.players.insert(String::from(player_key));

        // get all the insturments assigned to the player
        let instrument_keys = match self.state.players.by_key.get(player_key) {
            Some(player) => &player.instruments,
            None => return (),
        };

        // add staves and tracks to this flow
        for instrument_key in instrument_keys {
            let instrument = match self.state.instruments.get(instrument_key) {
                Some(instrument) => instrument,
                None => return (),
            };
            flow.add_instrument(instrument);
        }

        self.update();
        self.emit();
    }

    pub fn unassign_player(&mut self, flow_key: &str, player_key: &str) {
        // remove the player_key from the flow
        let flow = match self.state.flows.by_key.get_mut(flow_key) {
            Some(flow) => flow,
            None => return (),
        };
        flow.players.remove(player_key);

        // get all the insturments assigned to the player
        let instrument_keys = match self.state.players.by_key.get(player_key) {
            Some(player) => &player.instruments,
            None => return (),
        };

        // delete staves and tracks in this flow
        for instrument_key in instrument_keys {
            let stave_keys = match self.state.instruments.get(instrument_key) {
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

    pub fn get_ticks(&self, flow_key: &str) -> JsValue {
        let flow = match self.state.flows.by_key.get(flow_key) {
            Some(flow) => flow,
            None => {
                let empty = TickList {
                    list: Vec::new(),
                    width: 0.0,
                };
                return JsValue::from_serde(&empty).unwrap();
            }
        };
        let ticks = flow.get_ticks();
        JsValue::from_serde(&ticks).unwrap()
    }
}
