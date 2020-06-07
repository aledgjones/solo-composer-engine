use crate::state::Engine;
use crate::utils::storage::Storage;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    /**
     * Convert the enum to a string that can be easily stored in localStorage
     */
    pub fn to_string(&self) -> String {
        match self {
            ThemeMode::Light => String::from("light"),
            ThemeMode::Dark => String::from("dark"),
        }
    }

    /**
     * Convert a string to a ThemeMode used when retreiving the value from localStorage
     */
    pub fn from_string(mode: String) -> ThemeMode {
        if mode == "light" {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        }
    }
}

#[derive(Serialize)]
pub struct App {
    pub audition: bool,
    pub theme: ThemeMode,
}

impl App {
    pub fn new() -> App {
        if let Ok(store) = Storage::new() {
            let audition = store.get("sc/audition");
            let audition = match audition {
                Some(audition) => audition == "true",
                None => true,
            };
            let theme = store.get("sc/theme-mode");
            let theme = match theme {
                Some(mode) => ThemeMode::from_string(mode),
                None => ThemeMode::Dark,
            };
            App { audition, theme }
        } else {
            App {
                audition: true,
                theme: ThemeMode::Dark,
            }
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
    pub fn set_theme(&mut self, value: ThemeMode) {
        let string = value.to_string();
        self.state.app.theme = value;
        self.emit();
        if let Ok(store) = Storage::new() {
            store.set("sc/theme-mode", &string);
        }
    }
}
