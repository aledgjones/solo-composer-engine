use crate::state::score::instrument::defs::get_def;
use crate::state::score::stave::Stave;
use crate::state::score::track::Track;
use crate::state::Engine;
use crate::utils::shortid;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum PlayerType {
    Solo,
    Section,
}

#[derive(Serialize)]
pub struct Player {
    pub key: String,
    pub player_type: PlayerType,
    pub instruments: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Player {
    pub fn new(player_type: PlayerType) -> Player {
        Player {
            key: shortid(),
            player_type,
            instruments: Vec::new(),
            name: None,
        }
    }
}

#[derive(Serialize)]
pub struct Players {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Player>,
}

impl Players {
    pub fn new() -> Players {
        Players {
            order: Vec::new(),
            by_key: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_player(&mut self, player_type: PlayerType) -> JsValue {
        let player = Player::new(player_type);
        let player_key = player.key.clone();

        self.state.score.players.order.push(player_key.clone());

        // include new player in all flows
        for flow_key in &self.state.score.flows.order {
            let flow = self.state.score.flows.by_key.get_mut(flow_key);
            match flow {
                Some(flow) => {
                    flow.players.insert(player_key.clone());
                }
                None => {} // won't happen but we ignore if it does
            }
        }

        self.state
            .score
            .players
            .by_key
            .insert(player_key.clone(), player);

        self.emit();

        JsValue::from_str(&player_key)
    }

    /**
     * Assign instrument to player
     */
    pub fn assign_instrument(&mut self, player_key: &str, instrument_key: &str) -> JsValue {
        let player_key = String::from(player_key);
        let instrument_key = String::from(instrument_key);

        match self.state.score.players.by_key.get_mut(&player_key) {
            Some(player) => {
                // push the instrument_key into the player (assignment actually happens here)
                player.instruments.push(instrument_key.clone());
            }
            None => return JsValue::UNDEFINED,
        };

        // unwrap manually -- we can't just pass back the None to js as it expects a JsValue.
        let instrument = match self.state.score.instruments.get(&instrument_key) {
            Some(instrument) => instrument,
            None => return JsValue::UNDEFINED,
        };
        let instrument_def = match get_def(&instrument.id.as_str()) {
            Some(instrument_def) => instrument_def,
            None => return JsValue::UNDEFINED,
        };

        // add empty staves to each flow that contains the player
        for flow_key in &self.state.score.flows.order {
            match self.state.score.flows.by_key.get_mut(flow_key) {
                Some(flow) => {
                    if flow.players.contains(&player_key) {
                        for (i, stave_key) in instrument.staves.iter().enumerate() {
                            let track = Track::new();
                            let mut stave =
                                Stave::new(stave_key.clone(), &instrument_def.staves[i]);
                            stave.tracks.push(track.key.clone());

                            flow.tracks.insert(track.key.clone(), track);
                            flow.staves.insert(stave.key.clone(), stave);
                        }
                    }
                }
                None => {} // won't happen but we ignore if it does
            };
        }

        self.emit();
        JsValue::from_str(&player_key)
    }

    pub fn reorder_player(&mut self, old_index: usize, new_index: usize) {
        let removed = self.state.score.players.order.remove(old_index);
        self.state.score.players.order.insert(new_index, removed);
        self.emit();
    }

    pub fn remove_player(&mut self, player_key: &str) {
        // delete all instruments that this player holds
        let instrument_keys = match self.state.score.players.by_key.get(player_key) {
            // we need to clone so instrument_keys isn't a ref to self and so we can use it later
            Some(player) => player.instruments.clone(),
            None => return (),
        };
        for instrument_key in instrument_keys {
            self.remove_instrument(player_key, &instrument_key);
        }

        // remove the player from each flow
        for flow_key in &self.state.score.flows.order {
            let flow = match self.state.score.flows.by_key.get_mut(flow_key) {
                Some(flow) => flow,
                None => return (),
            };
            flow.players.remove(player_key);
        }

        // remove the player itself
        self.state.score.players.by_key.remove(player_key);
        self.state.score.players.order.retain(|e| e != player_key);

        self.emit();
    }
}
