use crate::state::entries::Entry;
use crate::state::Engine;
use crate::utils::duration::Duration;
use crate::utils::pitch::{Accidental, Pitch};
use crate::utils::shortid;
use wasm_bindgen::prelude::*;

/// These represent the audiable tones of the music.
/// They are never directly drawn in the score.
#[derive(Serialize)]
pub struct Tone {
    pub key: String,
    pub tick: u32,
    pub duration: Duration,
    pub pitch: Pitch, // the pitch that the clef sits on
}

impl Tone {
    pub fn new(key: String, tick: u32, duration: Duration, pitch: Pitch) -> Entry {
        Entry::Tone(Self {
            key,
            tick,
            duration,
            pitch,
        })
    }
}

#[wasm_bindgen]
impl Engine {
    /// Create a tone
    pub fn create_tone(
        &mut self,
        flow_key: &str,
        track_key: &str,
        tick: u32,
        duration: u32,
        pitch: u8,
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

        let track = match flow.tracks.get_mut(track_key) {
            Some(track) => track,
            None => return JsValue::UNDEFINED,
        };

        // we are now done with the entry, insert it back in
        track.insert(Tone::new(
            key.clone(),
            tick,
            Duration::new(duration),
            Pitch::new(pitch, Accidental::default(pitch)),
        ));

        self.update();
        self.emit();

        JsValue::from_str(key.as_str())
    }

    /// Update the tone
    pub fn update_tone(
        &mut self,
        flow_key: &str,
        track_key: &str,
        entry_key: &str,
        tick: u32,
        duration: u32,
        pitch: u8,
    ) {
        let flow = match self
            .state
            .score
            .flows
            .by_key
            .get_mut(&String::from(flow_key))
        {
            Some(flow) => flow,
            None => return (),
        };

        let track = match flow.tracks.get_mut(track_key) {
            Some(track) => track,
            None => return (),
        };

        // move the entry to the new start tick
        track.r#move(entry_key, tick);

        // update pitch and duration
        let tone = match track.entries.by_key.get_mut(entry_key) {
            Some(entry) => match entry {
                Entry::Tone(tone) => tone,
                _ => return (),
            },
            None => return (),
        };
        tone.pitch = Pitch::new(pitch, Accidental::default(pitch));
        tone.duration = Duration::new(duration);

        self.update();
        self.emit();
    }

    /// Remove the tone
    pub fn remove_tone(&mut self, flow_key: &str, track_key: &str, entry_key: &str) {
        let flow = match self
            .state
            .score
            .flows
            .by_key
            .get_mut(&String::from(flow_key))
        {
            Some(flow) => flow,
            None => return (),
        };

        let track = match flow.tracks.get_mut(track_key) {
            Some(track) => track,
            None => return (),
        };

        track.remove(entry_key);

        self.update();
        self.emit();
    }

    /// Slice a tone
    pub fn slice_tone(&mut self, flow_key: &str, track_key: &str, entry_key: &str, slice_at: u32) {
        let flow = match self
            .state
            .score
            .flows
            .by_key
            .get_mut(&String::from(flow_key))
        {
            Some(flow) => flow,
            None => return (),
        };

        let track = match flow.tracks.get_mut(track_key) {
            Some(track) => track,
            None => return (),
        };

        let old_tone = match track.remove(entry_key) {
            Some(entry) => match entry {
                Entry::Tone(tone) => tone,
                _ => return (),
            },
            None => return (),
        };

        track.insert(Tone::new(
            old_tone.key,
            old_tone.tick,
            Duration::new(slice_at - old_tone.tick),
            old_tone.pitch.clone(),
        ));

        track.insert(Tone::new(
            shortid(),
            slice_at,
            Duration::new(old_tone.duration.int - (slice_at - old_tone.tick)),
            old_tone.pitch.clone(),
        ));

        self.update();
        self.emit();
    }
}
