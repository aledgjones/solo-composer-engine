mod entries;
mod score;

use crate::state::score::Score;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Engine {
    listener: js_sys::Function,
    state: Score,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(callback: js_sys::Function) -> Engine {
        let engine = Engine {
            listener: callback,
            state: Score::new(),
        };
        engine.emit();
        engine
    }

    pub fn get(&self) -> JsValue {
        JsValue::from_serde(&self.state).unwrap()
    }

    fn emit(&self) {
        let this = JsValue::NULL;
        let state = JsValue::from_serde(&self.state).unwrap();
        let _ = self.listener.call1(&this, &state);
    }
}
