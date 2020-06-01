mod app;
mod meta;
mod theme;

use crate::state::app::App;
use crate::state::meta::Meta;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct State {
    meta: Meta,
    app: App,
}

impl State {
    fn new() -> State {
        State {
            meta: Meta::new(),
            app: App::new(),
        }
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

    fn emit(&self) {
        let this = JsValue::NULL;
        let state = JsValue::from_serde(&self.state).unwrap();
        let _ = self.listener.call1(&this, &state);
    }
}
