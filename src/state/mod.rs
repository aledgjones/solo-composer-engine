mod entries;
mod score;

use crate::state::score::Score;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct State {
    score: Score,
}

impl State {
    fn new() -> State {
        let score = Score::new();
        State { score }
    }
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
        Engine {
            listener: callback,
            state: State::new(),
        }
    }

    pub fn get(&self) -> JsValue {
        JsValue::from_serde(&self.state).unwrap()
    }

    fn emit(&mut self) {
        let this = JsValue::NULL;
        let state = JsValue::from_serde(&self.state).unwrap();
        let _ = self.listener.call1(&this, &state);
    }
}
