use crate::state::score::instrument::defs::INSTRUMENT_DEFS;
use crate::state::score::instrument::Instrument;
use crate::state::score::player::PlayerType;
use crate::state::Engine;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

fn append_instrument<'a>(
    map: &mut HashMap<&'a String, Vec<&'a String>>,
    instrument: &'a Instrument,
) {
    match map.get_mut(&instrument.long_name) {
        Some(entry) => {
            entry.push(&instrument.key);
        }
        None => {
            map.insert(&instrument.long_name, vec![&instrument.key]);
        }
    };
}

fn insert_counts<'a>(
    counts: &mut HashMap<&'a String, usize>,
    map: &HashMap<&'a String, Vec<&'a String>>,
) {
    for (_name, instrument_keys) in map {
        if instrument_keys.len() > 1 {
            for (i, instrument_key) in instrument_keys.iter().enumerate() {
                counts.insert(instrument_key, i + 1);
            }
        }
    }
}
#[derive(Serialize)]
struct FullPathReturn<'a> {
    path: &'a Vec<&'a str>,
    id: &'a str,
}

#[wasm_bindgen]
impl Engine {
    /**
     * Instrument Counts
     */
    pub fn counts(&self) -> JsValue {
        // long_name: instrument_keys[];
        let mut instruments_solo: HashMap<&String, Vec<&String>> = HashMap::new();
        let mut instruments_section: HashMap<&String, Vec<&String>> = HashMap::new();

        // collect all the instruments in order, dependant on player type (as numbered seperately)
        for player_key in &self.state.score.players.order {
            let player = match self.state.score.players.by_key.get(player_key) {
                Some(player) => player,
                None => return JsValue::UNDEFINED,
            };
            for instrument_key in &player.instruments {
                let instrument = match self.state.score.instruments.get(instrument_key) {
                    Some(instrument) => instrument,
                    None => return JsValue::UNDEFINED,
                };
                match player.player_type {
                    PlayerType::Solo => append_instrument(&mut instruments_solo, &instrument),
                    PlayerType::Section => append_instrument(&mut instruments_section, &instrument),
                };
            }
        }

        let mut counts: HashMap<&String, usize> = HashMap::new();
        insert_counts(&mut counts, &instruments_solo);
        insert_counts(&mut counts, &instruments_section);

        JsValue::from_serde(&counts).unwrap()
    }

    pub fn get_full_path_from_partial(&self, selection: JsValue) -> JsValue {
        let selection: Vec<String> = selection.into_serde().unwrap();

        let def = INSTRUMENT_DEFS.iter().find(|&def| {
            for (i, step) in selection.iter().enumerate() {
                if String::from(step) != def.path[i] {
                    return false; // we have a mismatched path -- this isn't what we're looking for
                }
            }
            true // even if we have a partial match only the first def we encounter is what we want
        });

        match def {
            Some(def) => JsValue::from_serde(&FullPathReturn {
                path: &&def.path,
                id: def.id,
            })
            .unwrap(),
            None => JsValue::UNDEFINED,
        }
    }

    /**
     * Get a tree of instruments from a (possibly incomplete) path
     */
    pub fn def_tree(&self, selection: JsValue) -> JsValue {
        let selection: Vec<String> = selection.into_serde().unwrap();

        let mut ignore: HashSet<&str> = HashSet::new();
        let mut tree: [Vec<&str>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for def in INSTRUMENT_DEFS.iter() {
            for (i, step) in def.path.iter().enumerate() {
                if !ignore.contains(def.id) {
                    if !tree[i].contains(step) {
                        tree[i].push(step);
                    }
                    if step.to_string() != selection[i] {
                        ignore.insert(def.id);
                    }
                }
            }
        }

        JsValue::from_serde(&tree).unwrap()
    }
}
