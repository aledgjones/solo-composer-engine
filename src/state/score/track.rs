use crate::entries::time_signature::TimeSignature;
use crate::entries::{Entry, EntryContent};
use crate::utils::shortid;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Entries {
    pub by_tick: HashMap<u32, Vec<String>>,
    pub by_key: HashMap<String, Entry>, // we can iterate the hashmap directly, so no order/by_key needed
}

#[derive(Serialize)]
pub struct Track {
    pub key: String,
    pub entries: Entries,
}

impl Track {
    pub fn new() -> Track {
        Track {
            key: shortid(),
            entries: Entries {
                by_tick: HashMap::new(),
                by_key: HashMap::new(),
            },
        }
    }
    pub fn insert(&mut self, entry: Entry) {
        let tick = self.entries.by_tick.entry(entry.tick).or_insert(Vec::new());
        tick.push(entry.key.clone());
        self.entries.by_key.insert(entry.key.clone(), entry);
    }
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_time_signature_at_tick(&self, tick: u32) -> Option<&TimeSignature> {
        let entry_keys = match self.entries.by_tick.get(&tick) {
            Some(entries) => entries,
            None => return None,
        };
        for key in entry_keys.iter() {
            match self.entries.by_key.get(key) {
                Some(entry) => match &entry.content {
                    EntryContent::TimeSignature(time_signature) => return Some(&time_signature),
                    _ => (),
                },
                None => (),
            }
        }

        None
    }
}
