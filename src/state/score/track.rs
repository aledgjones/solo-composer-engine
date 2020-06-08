use crate::entries::Entry;
use crate::utils::shortid;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TrackEntries {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Entry>,
}

impl TrackEntries {
    pub fn new() -> TrackEntries {
        TrackEntries {
            order: Vec::new(),
            by_key: HashMap::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Track {
    pub key: String,
    pub entries: TrackEntries,
}

impl Track {
    pub fn new() -> Track {
        Track {
            key: shortid(),
            entries: TrackEntries::new(),
        }
    }
}
