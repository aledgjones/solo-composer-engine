use crate::state::theme::Theme;
use crate::state::Engine;
use crate::utils::storage::Storage;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct App {
    pub audition: bool,
    pub theme: Theme,
}

impl App {
    pub fn new() -> App {
        let audition = if let Ok(store) = Storage::new() {
            let value = store.get("sc/audition");
            match value {
                Some(audition) => audition == "true",
                None => true,
            }
        } else {
            // if storage fails fall back to default
            true
        };
        App {
            audition,
            theme: Theme::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_audition(&mut self, value: bool) {
        self.state.app.audition = value;
        self.emit();
        if let Ok(store) = Storage::new() {
            let value = if value == true {
                String::from("true")
            } else {
                String::from("false")
            };
            store.set("sc/audition", &value);
        }
    }
}
