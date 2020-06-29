mod app;
mod entries;
mod playback;
mod score;
mod ui;

use crate::state::app::App;
use crate::state::playback::Playback;
use crate::state::score::Score;
use crate::state::ui::Ui;

use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct State {
    app: App,
    playback: Playback,
    score: Score,
    ui: Ui,
}

impl State {
    fn new() -> State {
        State {
            app: App::new(),
            playback: Playback::new(),
            score: Score::new(),
            ui: Ui::new(),
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
