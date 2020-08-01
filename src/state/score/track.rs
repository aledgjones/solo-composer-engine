use crate::state::entries::absolute_tempo::AbsoluteTempo;
use crate::state::entries::time_signature::TimeSignature;
use crate::state::entries::Entry;
use crate::utils::shortid;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Entries {
    pub by_tick: HashMap<u32, Vec<String>>,
    pub by_key: HashMap<String, Entry>, // we can iterate the hashmap directly, so no order/by_key needed
}

#[derive(Serialize, Deserialize)]
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

    /// Move an entry to a new tick
    pub fn r#move(&mut self, key: &str, new_tick: u32) {
        let entry = match self.entries.by_key.get_mut(key) {
            Some(entry) => entry,
            None => return (),
        };

        let old_tick = entry.tick();

        // move the entry tp the new tick only if it has actually moved
        if old_tick != new_tick {
            entry.set_tick(new_tick);
            // move the entry key to the new tick
            match self.entries.by_tick.get_mut(&old_tick) {
                Some(keys) => {
                    keys.retain(|item| item != key);
                }
                None => (),
            };
            let tick = self.entries.by_tick.entry(new_tick).or_insert(Vec::new());
            tick.push(String::from(key));
        }
    }

    /// remove an entry and return the removed entry
    pub fn remove(&mut self, key: &str) -> Option<Entry> {
        let entry = match self.entries.by_key.get(key) {
            Some(entry) => entry,
            None => return None,
        };

        match self.entries.by_tick.get_mut(&entry.tick()) {
            Some(keys) => {
                keys.retain(|item| item != key);
            }
            None => (),
        };
        self.entries.by_key.remove(key)
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

    /// Returns the previous time signature entry *on* or *before* a given tick if it exists
    pub fn get_time_signature_on_or_before_tick(&self, tick: u32) -> Option<&TimeSignature> {
        // tick + 1 as the top value is not inclusive and we want to include the current tick.
        // ie 0..5 == 0,1,2,3,4;
        for i in (0..tick + 1).rev() {
            match self.get_time_signature_at_tick(i) {
                Some(time_signature) => return Some(time_signature),
                None => (),
            };
        }

        None
    }

    /// Returns the time signature entry at a given tick if it exists
    pub fn get_absolute_tempo_at_tick(&self, tick: u32) -> Option<&AbsoluteTempo> {
        let entry_keys = match self.entries.by_tick.get(&tick) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys.iter() {
            match self.entries.by_key.get(key) {
                Some(entry) => match entry {
                    Entry::AbsoluteTempo(tempo) => return Some(tempo),
                    _ => (),
                },
                None => (),
            }
        }

        None
    }
}
