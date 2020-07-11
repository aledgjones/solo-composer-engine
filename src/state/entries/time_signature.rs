use crate::state::entries::Entry;
use crate::state::Engine;
use crate::utils::measurements::{BoundingBox, Padding, Spaces};
use crate::utils::shortid;
use wasm_bindgen::prelude::*;

enum TimeSignatureType {
    Simple,
    Compound,
    Complex,
    Open,
}

#[wasm_bindgen]
#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum TimeSignatureDrawType {
    Hidden,          // always hidden
    Normal,          // open time sig as 'X'
    CommonTime,      // 'C'
    SplitCommonTime, // '¢'
}

#[derive(Debug, Serialize)]
pub struct TimeSignature {
    pub key: String,
    pub tick: u32,
    pub beats: u8,
    pub beat_type: u8,
    pub draw_type: TimeSignatureDrawType,
    pub groupings: Vec<u8>, // optional, we can always revert to defaults
}

impl TimeSignature {
    pub fn new(
        key: String,
        tick: u32,
        beats: u8,
        beat_type: u8,
        draw_type: TimeSignatureDrawType,
        groupings: Option<Vec<u8>>,
    ) -> Entry {
        Entry::TimeSignature(Self {
            key,
            tick,
            beats,
            beat_type,
            groupings: match groupings {
                Some(groupings) => groupings,
                None => TimeSignature::groupings(beats),
            },
            draw_type,
        })
    }

    /// Return the time signature type Open, Compound, Simple or Complex.
    fn kind(&self) -> TimeSignatureType {
        TimeSignature::kind_from_beats(self.beats)
    }

    /// Return the time signature type Open, Compound, Simple or Complex.
    fn kind_from_beats(beats: u8) -> TimeSignatureType {
        if beats == 0 {
            TimeSignatureType::Open
        } else if beats > 3 && beats % 3 == 0 {
            TimeSignatureType::Compound
        } else if beats == 1 || beats == 2 || beats == 3 || beats == 4 {
            TimeSignatureType::Simple
        } else {
            TimeSignatureType::Complex
        }
    }

    fn groupings(beats: u8) -> Vec<u8> {
        if beats > 0 && beats <= 3 {
            vec![1; beats as usize]
        } else {
            match TimeSignature::kind_from_beats(beats) {
                TimeSignatureType::Simple => vec![2; (beats as usize) / 2],
                TimeSignatureType::Compound => vec![3; (beats as usize) / 3],
                TimeSignatureType::Complex => {
                    let mut out: Vec<u8> = Vec::new();
                    let mut remaining = beats;
                    while remaining > 4 {
                        out.push(3);
                        remaining = remaining - 3;
                    }
                    out.push(remaining);
                    out
                }
                TimeSignatureType::Open => vec![],
            }
        }
    }

    /// Get the number of ticks per the time signatures bar
    pub fn ticks_per_bar(&self, subdivisions: u8) -> u8 {
        self.ticks_per_beat(subdivisions) * self.beats
    }

    /// Get the number of ticks per the time signatures beat type
    pub fn ticks_per_beat(&self, subdivisions: u8) -> u8 {
        self.ticks_per_beat_type(subdivisions, self.beat_type)
    }

    /// Get the number of ticks per an arbitary beat type
    pub fn ticks_per_beat_type(&self, subdivisions: u8, beat_type: u8) -> u8 {
        (subdivisions as f32 / (beat_type as f32 / 4 as f32)) as u8
    }

    // Returns true if the tick is on a beat
    pub fn is_on_beat(&self, tick: u32, subdivisions: u8) -> bool {
        self.is_on_beat_type(tick, subdivisions, self.beat_type)
    }

    /// Return true if a tick is on an arbitrary beat type
    pub fn is_on_beat_type(&self, tick: u32, subdivisions: u8, beat_type: u8) -> bool {
        let ticks_per_beat = self.ticks_per_beat_type(subdivisions, beat_type) as u32;
        ((tick - self.tick) % ticks_per_beat) == 0
    }

    // Returns true if the tick is on the first beat of the bar
    pub fn is_on_first_beat(&self, tick: u32, subdivisions: u8) -> bool {
        match self.kind() {
            TimeSignatureType::Open => tick == self.tick,
            _ => ((tick - self.tick) % self.ticks_per_bar(subdivisions) as u32) == 0,
        }
    }

    // Returns true is the tick is on a beat group boundry
    pub fn is_on_grouping_boundry(&self, tick: u32, subdivisions: u8) -> bool {
        match self.kind() {
            TimeSignatureType::Open => false,
            _ => {
                let ticks_per_beat = self.ticks_per_beat_type(subdivisions, self.beat_type);
                let bar_length = (ticks_per_beat * self.beats) as u32;
                let distance_from_first_beat = (tick - self.tick) % bar_length;

                if distance_from_first_beat == 0 {
                    return true;
                }

                let mut offset: u32 = 0;
                for group in &self.groupings {
                    offset += (group * ticks_per_beat) as u32;
                    if distance_from_first_beat == offset {
                        return true;
                    }
                }

                false
            }
        }
    }

    pub fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: Spaces(0.75),
            height: Spaces(4.0),
            padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_time_signature(
        &mut self,
        flow_key: &str,
        tick: u32,
        beats: u8,
        beat_type: u8,
        draw_type: TimeSignatureDrawType,
        groupings: Option<Vec<u8>>,
    ) -> JsValue {
        // we want to be able to return this at the end
        let key = shortid();

        let flow = match self
            .state
            .score
            .flows
            .by_key
            .get_mut(&String::from(flow_key))
        {
            Some(flow) => flow,
            None => return JsValue::UNDEFINED,
        };

        // if there is already a time isg at this tick we remove it
        let old_key = match flow.master.get_time_signature_at_tick(tick) {
            Some(time_signature) => Some(time_signature.key.clone()),
            None => None,
        };

        match old_key {
            Some(key) => {
                flow.master.remove(&key);
            }
            None => (),
        };

        // we insert the new time sig
        let entry = TimeSignature::new(key.clone(), tick, beats, beat_type, draw_type, groupings);

        // extract the time sig itself out the Entry, we need it's methods to work with
        let time_signature = match &entry {
            Entry::TimeSignature(time_signature) => time_signature,
            _ => return JsValue::UNDEFINED, // will never happen, something has gone horribly wrong!
        };

        // we want to create full bars when we insert time sigs,
        // however open time sigs don't have bar lengths in reality so ignore this step
        match time_signature.kind() {
            TimeSignatureType::Open => (),
            _ => {
                let bar_length = time_signature.ticks_per_bar(flow.subdivisions) as u32;

                // calculate how may ticks we have filled of the last bar before
                let overflow = match flow.master.get_time_signature_after_tick(tick, flow.length) {
                    Some(next_time_signature) => (next_time_signature.tick - tick) % bar_length,
                    None => ((flow.length - tick) % bar_length),
                };

                if overflow > 0 {
                    // add aditional ticks to flow length to make full bars
                    flow.length += bar_length - overflow;
                }

                // offset all remaining time sigs by remainder
                for i in tick + 1..flow.length {
                    let key = match flow.master.get_time_signature_at_tick(i) {
                        Some(time_signature) => Some(time_signature.key.clone()),
                        None => None,
                    };
                    match key {
                        Some(key) => {
                            flow.master.r#move(&key, i + (bar_length - overflow));
                        }
                        None => (),
                    }
                }
            }
        }

        // we are now done with the entry, insert it back in
        flow.master.insert(entry);

        self.state.ticks.insert(flow.key.clone(), flow.get_ticks());

        self.update();
        self.emit();

        JsValue::from_str(key.as_str())
    }
}
