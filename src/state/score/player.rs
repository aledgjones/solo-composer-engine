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
        let player_key = player.key.clone(); // this will be the return value

        self.state.score.players.order.push(player.key.clone());

        // include new player in all flows
        for flow_key in &self.state.score.flows.order {
            let flow = self.state.score.flows.by_key.get_mut(flow_key);
            match flow {
                Some(flow) => {
                    flow.players.insert(player.key.clone());
                }
                None => {} // won't happen but we ignore if it does
            }
        }

        self.state
            .score
            .players
            .by_key
            .insert(player.key.clone(), player);

        self.emit();

        JsValue::from_str(&player_key)
    }

    /**
     * Assign instrument to player
     */
    pub fn assign_instrument(&mut self, player_key: &str, instrument_key: &str) -> JsValue {
        let player_key = String::from(player_key);
        let player = self.state.score.players.by_key.get_mut(&player_key);
        match player {
            Some(player) => {
                let instrument_key = String::from(instrument_key);
                player.instruments.push(instrument_key);
            }
            None => return JsValue::UNDEFINED,
        }
        self.emit();
        JsValue::TRUE
    }
}
