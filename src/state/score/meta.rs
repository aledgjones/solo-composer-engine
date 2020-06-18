use crate::state::Engine;
use js_sys::Date;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Meta {
    pub title: String,
    pub subtitle: String,
    pub composer: String,
    pub arranger: String,
    pub lyricist: String,
    pub copyright: String,
    pub created: f64,
    pub modified: f64,
}

impl Meta {
    pub fn new() -> Meta {
        Meta {
            title: String::from(""),
            subtitle: String::from(""),
            composer: String::from(""),
            arranger: String::from(""),
            lyricist: String::from(""),
            copyright: String::from(""),
            created: Date::now(),
            modified: Date::now(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_title(&mut self, value: String) {
        self.state.score.meta.title = value;
        self.update();
        self.emit();
    }
    pub fn set_subtitle(&mut self, value: String) {
        self.state.score.meta.subtitle = value;
        self.update();
        self.emit();
    }
    pub fn set_composer(&mut self, value: String) {
        self.state.score.meta.composer = value;
        self.update();
        self.emit();
    }
    pub fn set_arranger(&mut self, value: String) {
        self.state.score.meta.arranger = value;
        self.update();
        self.emit();
    }
    pub fn set_lyricist(&mut self, value: String) {
        self.state.score.meta.lyricist = value;
        self.update();
        self.emit();
    }
    pub fn set_copyright(&mut self, value: String) {
        self.state.score.meta.copyright = value;
        self.update();
        self.emit();
    }
}
