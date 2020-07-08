use crate::state::score::instrument::defs::INSTRUMENT_DEFS;
use crate::state::score::instrument::Instrument;
use crate::state::score::player::PlayerType;
use crate::state::Engine;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

fn append_instruments<'a>(
    map: &mut HashMap<String, Vec<String>>,
    instrument_keys: &Vec<String>,
    instruments: &'a mut HashMap<String, Instrument>,
) {
    for instrument_key in instrument_keys {
        let instrument = match instruments.get_mut(instrument_key) {
            Some(instrument) => instrument,
            None => return (),
        };
        // we need to clear any previously calculated counts
        instrument.count = None;
        // build counts here
        match map.get_mut(&instrument.long_name) {
            Some(entry) => {
                entry.push(instrument.key.clone());
            }
            None => {
                map.insert(instrument.long_name.clone(), vec![instrument.key.clone()]);
            }
        };
    }
}

fn insert_counts(engine: &mut Engine, map: &HashMap<String, Vec<String>>) {
    for (_name, instrument_keys) in map {
        if instrument_keys.len() > 1 {
            for (i, instrument_key) in instrument_keys.iter().enumerate() {
                match engine.state.score.instruments.get_mut(instrument_key) {
                    Some(instrument) => instrument.count = Some(i as u8 + 1),
                    None => (),
                };
            }
        }
    }
}
#[derive(Serialize)]
struct FullPathReturn<'a> {
    path: &'a Vec<&'a str>,
    id: &'a str,
}

/// Get a full path to def from partial path
#[wasm_bindgen]
pub fn get_full_path_from_partial(selection: &JsValue) -> JsValue {
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

/// Get a tree of instruments from a (possibly incomplete) path
#[wasm_bindgen]
pub fn def_tree(selection: &JsValue) -> JsValue {
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

/**
 * Instrument Counts
 * Will be caculated even if they are not displayed in the UI layer
 */
pub fn calc_counts(engine: &mut Engine) {
    // long_name: instrument_keys[];
    let mut instruments_solo: HashMap<String, Vec<String>> = HashMap::new();
    let mut instruments_section: HashMap<String, Vec<String>> = HashMap::new();

    // collect all the instruments in order, dependant on player type (as numbered seperately)
    for player_key in &engine.state.score.players.order {
        let player = match engine.state.score.players.by_key.get(player_key) {
            Some(player) => player,
            None => return (),
        };
        match player.player_type {
            PlayerType::Solo => {
                append_instruments(
                    &mut instruments_solo,
                    &player.instruments,
                    &mut engine.state.score.instruments,
                );
            }
            PlayerType::Section => {
                append_instruments(
                    &mut instruments_section,
                    &player.instruments,
                    &mut engine.state.score.instruments,
                );
            }
        }
    }

    insert_counts(engine, &instruments_solo);
    insert_counts(engine, &instruments_section);
}
