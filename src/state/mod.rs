mod entries;
mod score;

use crate::state::score::flow::TickList;
use crate::state::score::Score;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    score: Score,
    ticks: HashMap<String, TickList>,
}

#[wasm_bindgen]
pub struct Engine {
    listener: js_sys::Function,
    state: State,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(callback: js_sys::Function) -> Engine {
        let mut engine = Engine {
            listener: callback,
            state: State {
                score: Score::new(),
                ticks: HashMap::new(),
            },
        };
        for (key, flow) in &engine.state.score.flows.by_key {
            engine.state.ticks.insert(key.clone(), flow.calc_ticks());
        }
        engine.emit();
        engine
    }

    pub fn export(&self) -> JsValue {
        JsValue::from_serde(&self.state.score).unwrap()
    }

    pub fn import(&mut self, state: JsValue) {
        self.state.score = state.into_serde().unwrap();
        for (key, flow) in &self.state.score.flows.by_key {
            self.state.ticks.insert(key.clone(), flow.calc_ticks());
        }
        self.emit();
    }

    fn emit(&self) {
        let this = JsValue::NULL;
        let state = JsValue::from_serde(&self.state).unwrap();
        let _ = self.listener.call1(&this, &state);
    }
}
