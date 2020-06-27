use crate::utils::generic::{BoundingBox, Padding, Spaces};
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
    Normal,          // open time sig hidden, others as normal
    CommonTime,      // 'C'
    SplitCommonTime, // '¢'
    Free,            // 'X'
}

#[derive(Debug, Serialize)]
pub struct TimeSignature {
    pub at: u32,
    pub subdivisions: u8,
    pub beats: u8,
    pub beat_type: u8,
    pub draw_type: TimeSignatureDrawType,
    pub groupings: Vec<u8>, // optional, we can always revert to defaults
}

impl TimeSignature {
    pub fn new(
        at: u32,
        subdivisions: u8,
        beats: u8,
        beat_type: u8,
        draw_type: TimeSignatureDrawType,
        groupings: Option<Vec<u8>>,
    ) -> TimeSignature {
        TimeSignature {
            at,
            subdivisions,
            beats,
            beat_type,
            groupings: match groupings {
                Some(groupings) => groupings,
                None => TimeSignature::groupings(beats),
            },
            draw_type,
        }
    }
    fn kind(beats: u8) -> TimeSignatureType {
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
        match TimeSignature::kind(beats) {
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

    /// Get the number of ticks per the time signatures beat type
    pub fn ticks_per_beat(&self) -> u8 {
        self.ticks_per_beat_type(self.beat_type)
    }

    /// Get the number of ticks per an arbitary beat type
    pub fn ticks_per_beat_type(&self, beat_type: u8) -> u8 {
        self.subdivisions / (beat_type / 4)
    }

    // Returns true if the tick is on a beat
    pub fn is_on_beat(&self, tick: u32) -> bool {
        self.is_on_beat_type(tick, self.beat_type)
    }

    /// Return true if a tick is on an arbitrary beat type
    pub fn is_on_beat_type(&self, tick: u32, beat_type: u8) -> bool {
        let ticks_per_beat = self.ticks_per_beat_type(beat_type) as u32;
        ((tick - self.at) % ticks_per_beat) == 0
    }

    // Returns true if the tick is on the first beat of the bar
    pub fn is_on_first_beat(&self, tick: u32) -> bool {
        // always return false for open time signatures
        if self.beats == 0 {
            false
        } else {
            let bar_length = (self.ticks_per_beat_type(self.beat_type) * self.beats) as u32;
            ((tick - self.at) % bar_length) == 0
        }
    }

    // Returns true is the tick is on a beat group boundry
    pub fn is_on_grouping_boundry(&self, tick: u32) -> bool {
        // always return false for open time signatures
        if self.beats == 0 {
            false
        } else {
            let ticks_per_beat = self.ticks_per_beat_type(self.beat_type);
            let bar_length = (ticks_per_beat * self.beats) as u32;
            let distance_from_first_beat = (tick - self.at) % bar_length;

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

    pub fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: Spaces(0.75),
            height: Spaces(4.0),
            padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_free() -> Result<(), String> {
        match TimeSignature::kind(4) {
            TimeSignatureType::Simple => Ok(()),
            _ => Err(String::from("4 is not simple time")),
        }
    }
}
