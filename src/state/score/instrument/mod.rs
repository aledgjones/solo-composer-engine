pub mod defs;
pub mod utils;

use crate::state::score::instrument::defs::get_def;
use crate::state::Engine;
use crate::utils::shortid;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Instrument {
    pub key: String,
    pub id: String,
    pub long_name: String,
    pub short_name: String,
    pub staves: Vec<String>,
}

#[derive(Serialize)]
struct CreateInstrumentReturn {
    key: String,
    patches: HashMap<&'static str, &'static str>,
}

#[wasm_bindgen]
impl Engine {
    /**
     * Create an instrument
     */
    pub fn create_instrument(&mut self, id: &str) -> JsValue {
        let def = match get_def(&id) {
            Some(def) => def,
            None => return JsValue::UNDEFINED,
        };
        let instrument = Instrument {
            key: shortid(),
            id: String::from(id),
            long_name: String::from(def.long_name),
            short_name: String::from(def.short_name),
            staves: def
                .staves
                .iter()
                .map(|_| shortid())
                .collect::<Vec<String>>(),
        };
        let return_value = CreateInstrumentReturn {
            key: instrument.key.clone(),  // return the newly created instrument's key
            patches: def.patches.clone(), // we are actually going to deal with playback js side
        };
        self.state
            .score
            .instruments
            .insert(instrument.key.clone(), instrument);
        self.update();
        self.emit();
        JsValue::from_serde(&return_value).unwrap()
    }

    pub fn reorder_instrument(&mut self, player_key: &str, old_index: usize, new_index: usize) {
        match self.state.score.players.by_key.get_mut(player_key) {
            Some(player) => {
                let removed = player.instruments.remove(old_index);
                player.instruments.insert(new_index, removed);
            }
            None => (),
        }
        self.update();
        self.emit();
    }

    pub fn remove_instrument(&mut self, player_key: &str, instrument_key: &str) {
        // remove from the player entry
        match self.state.score.players.by_key.get_mut(player_key) {
            Some(player) => {
                player.instruments.retain(|e| e != instrument_key);
            }
            None => (),
        };

        let stave_keys = match self.state.score.instruments.get(instrument_key) {
            Some(instrument) => &instrument.staves,
            None => return (),
        };

        for flow_key in &self.state.score.flows.order {
            let flow = match self.state.score.flows.by_key.get_mut(flow_key) {
                Some(flow) => flow,
                None => return (),
            };

            if flow.players.contains(player_key) {
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
        }

        self.state.score.instruments.remove(instrument_key);
        self.update();
        self.emit();
    }
}
