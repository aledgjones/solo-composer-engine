use crate::state::score::instrument::defs::StaveDef;
use crate::state::score::track::Track;

#[derive(Serialize, Deserialize)]
pub struct Stave {
    pub key: String,
    pub lines: Vec<u8>,
    pub master: Track,
    pub tracks: Vec<String>,
}

impl Stave {
    pub fn new(key: String, stave_def: &StaveDef) -> Stave {
        Stave {
            key,
            lines: stave_def.lines.clone(),
            master: Track::new(),
            tracks: Vec::new(),
        }
    }
}
