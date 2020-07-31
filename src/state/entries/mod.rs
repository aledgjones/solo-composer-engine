pub mod absolute_tempo;
pub mod barline;
pub mod clef;
pub mod time_signature;
pub mod tone;

use absolute_tempo::AbsoluteTempo;
use barline::Barline;
use clef::Clef;
use time_signature::TimeSignature;
use tone::Tone;

#[derive(Serialize)]
pub enum Entry {
    Barline(Barline),
    Clef(Clef),
    TimeSignature(TimeSignature),
    Tone(Tone),
    AbsoluteTempo(AbsoluteTempo),
}

impl Entry {
    /// Get the entries key without having to manually unwrap the entry
    pub fn key(&self) -> String {
        match self {
            Entry::Barline(barline) => barline.key.clone(),
            Entry::Clef(clef) => clef.key.clone(),
            Entry::TimeSignature(time_signature) => time_signature.key.clone(),
            Entry::Tone(tone) => tone.key.clone(),
            Entry::AbsoluteTempo(tempo) => tempo.key.clone(),
        }
    }

    /// Get the entries tick without having to manually unwrap the entry
    pub fn tick(&self) -> u32 {
        match self {
            Entry::Barline(barline) => barline.tick,
            Entry::Clef(clef) => clef.tick,
            Entry::TimeSignature(time_signature) => time_signature.tick,
            Entry::Tone(tone) => tone.tick,
            Entry::AbsoluteTempo(tempo) => tempo.tick,
        }
    }

    /// Updates the entries tick value
    ///
    /// This shouldn't be called directly. It will most likely be called via the
    ///  Track interface.
    pub fn set_tick(&mut self, tick: u32) {
        match self {
            Entry::Barline(barline) => barline.tick = tick,
            Entry::Clef(clef) => clef.tick = tick,
            Entry::TimeSignature(time_signature) => time_signature.tick = tick,
            Entry::Tone(tone) => tone.tick = tick,
            Entry::AbsoluteTempo(tempo) => tempo.tick = tick,
        }
    }
}
