pub mod barline;
pub mod time_signature;

use barline::Barline;
use time_signature::TimeSignature;

#[derive(Debug, Serialize)]
pub enum Entry {
    Barline(Barline),
    TimeSignature(TimeSignature),
}

impl Entry {
    pub fn key(&self) -> String {
        match self {
            Entry::Barline(barline) => barline.key.clone(),
            Entry::TimeSignature(time_signature) => time_signature.key.clone(),
        }
    }
    pub fn tick(&self) -> u32 {
        match self {
            Entry::Barline(barline) => barline.tick,
            Entry::TimeSignature(time_signature) => time_signature.tick,
        }
    }
    pub fn set_tick(&mut self, tick: u32) {
        match self {
            Entry::Barline(barline) => barline.tick = tick,
            Entry::TimeSignature(time_signature) => time_signature.tick = tick,
        }
    }
}
