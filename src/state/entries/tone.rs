use crate::state::entries::Entry;
use crate::state::Engine;
use crate::utils::duration::Duration;
use crate::utils::pitch::{Accidental, Pitch};
use crate::utils::shortid;
use crate::utils::velocity::Velocity;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u8)]
pub enum Articulation {
    None,
    Staccato,
    Staccatissimo,
    Tenuto,
    StaccatoTenuto,
}

/// These represent the audiable tones of the music.
/// They are never directly drawn in the score.
#[derive(Serialize, Deserialize)]
pub struct Tone {
    pub key: String,
    pub tick: u32,
    pub duration: Duration,
    pub pitch: Pitch, // the pitch that the clef sits on
    pub velocity: Velocity,
    pub articulation: Articulation,
}

impl Tone {
    pub fn new(
        key: String,
        tick: u32,
        duration: Duration,
        pitch: Pitch,
        velocity: Velocity,
        articulation: Articulation,
    ) -> Entry {
        Entry::Tone(Self {
            key,
            tick,
            duration,
            pitch,
            velocity,
            articulation,
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
        velocity: u8,
        articulation: Articulation,
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
            Velocity::new(velocity),
            articulation,
        ));

        self.state.score.meta.set_modified();
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
        articulation: Articulation,
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
        tone.articulation = articulation;

        self.state.score.meta.set_modified();
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

        self.state.score.meta.set_modified();
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
            old_tone.pitch,
            old_tone.velocity,
            old_tone.articulation,
        ));

        track.insert(Tone::new(
            shortid(),
            slice_at,
            Duration::new(old_tone.duration.int - (slice_at - old_tone.tick)),
            old_tone.pitch,
            old_tone.velocity,
            old_tone.articulation,
        ));

        self.state.score.meta.set_modified();
        self.emit();
    }
}
