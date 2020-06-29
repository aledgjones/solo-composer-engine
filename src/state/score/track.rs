use crate::state::entries::time_signature::TimeSignature;
use crate::state::entries::Entry;
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

    /// Insert and entry into the track
    pub fn insert(&mut self, entry: Entry) {
        let tick = self
            .entries
            .by_tick
            .entry(entry.tick())
            .or_insert(Vec::new());
        tick.push(entry.key());
        self.entries.by_key.insert(entry.key(), entry);
    }

    /// remove an entry and return the removed entry
    pub fn remove(&mut self, tick: &u32, key: &String) -> Option<Entry> {
        match self.entries.by_tick.get_mut(tick) {
            Some(keys) => {
                keys.retain(|item| item != key);
            }
            None => (),
        };
        self.entries.by_key.remove(key)
    }

    pub fn r#move(&mut self, old_tick: &u32, new_tick: u32, key: &String) {
        let entry = self.remove(old_tick, key);
        let entry = match entry {
            Some(mut entry) => {
                entry.set_tick(new_tick);
                entry
            }
            None => return (),
        };
        self.insert(entry);
    }

    /// Returns the time signature entry at a given tick if it exists
    pub fn get_time_signature_at_tick(&self, tick: u32) -> Option<&TimeSignature> {
        let entry_keys = match self.entries.by_tick.get(&tick) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys.iter() {
            match self.entries.by_key.get(key) {
                Some(entry) => match entry {
                    Entry::TimeSignature(time_signature) => return Some(time_signature),
                    _ => (),
                },
                None => (),
            }
        }

        None
    }

    /// Returns the next time signature entry *after* a given tick if it exists
    pub fn get_time_signature_after_tick(&self, tick: u32, length: u32) -> Option<&TimeSignature> {
        for i in tick + 1..length {
            match self.get_time_signature_at_tick(i) {
                Some(time_signature) => return Some(time_signature),
                None => (),
            };
        }

        None
    }
}
