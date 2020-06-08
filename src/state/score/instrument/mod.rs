pub mod defs;

use crate::state::score::instrument::defs::INSTRUMENT_DEFS;
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
        let def = INSTRUMENT_DEFS.get(&String::from(id));
        let def = match def {
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
        self.emit();
        JsValue::from_serde(&return_value).unwrap()
    }
}
