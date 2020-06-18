mod config;
mod engrave;
mod flow;
mod instrument;
mod meta;
mod player;
mod stave;
mod track;

use crate::state::score::config::Config;
use crate::state::score::engrave::{Engrave, LayoutType};
use crate::state::score::flow::Flows;
use crate::state::score::instrument::Instrument;
use crate::state::score::meta::Meta;
use crate::state::score::player::Players;
use crate::state::Engine;
use js_sys::Date;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Score {
    pub meta: Meta,
    pub config: Config,
    pub engrave: HashMap<String, Engrave>,
    pub flows: Flows,
    pub players: Players,
    pub instruments: HashMap<String, Instrument>,
}

impl Score {
    pub fn new() -> Score {
        let mut engrave = HashMap::new();

        let score = Engrave::new(LayoutType::Score, String::from("Score"));
        engrave.insert(score.key.clone(), score);
        let part = Engrave::new(LayoutType::Part, String::from("Part"));
        engrave.insert(part.key.clone(), part);

        Score {
            meta: Meta::new(),
            config: Config::new(),
            engrave: engrave,
            flows: Flows::new(),
            players: Players::new(),
            instruments: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    fn update(&mut self) {
        self.state.score.meta.modified = Date::now();
    }
}
