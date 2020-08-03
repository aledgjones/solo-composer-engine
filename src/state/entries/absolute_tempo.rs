use crate::state::entries::Entry;
use crate::state::Engine;
use crate::utils::duration::NoteDuration;
use crate::utils::measurements::{BoundingBox, Padding, Spaces};
use crate::utils::shortid;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AbsoluteTempo {
    pub key: String,
    pub tick: u32,
    pub normalized_bpm: f64,

    // written representation
    pub text: String,
    pub beat_type: NoteDuration,
    pub dotted: u8,
    pub bpm: u32,

    // written representation config
    pub parenthesis_visible: bool,
    pub text_visible: bool,
    pub bpm_visible: bool,
}

impl AbsoluteTempo {
    pub fn new(
        subdivisions: u8,
        key: String,
        tick: u32,
        text: String,
        beat_type: NoteDuration,
        dotted: u8,
        bpm: u32,
        parenthesis_visible: bool,
        text_visible: bool,
        bpm_visible: bool,
    ) -> Entry {
        Entry::AbsoluteTempo(Self {
            key,
            tick,
            normalized_bpm: AbsoluteTempo::normalize_bpm(subdivisions, &beat_type, dotted, bpm),

            text,
            beat_type,
            dotted,
            bpm,

            text_visible,
            bpm_visible,
            parenthesis_visible,
        })
    }

    /// Get a tempo normalized in quarters per minute
    ///
    /// Example h. = 100 => 300bpm
    fn normalize_bpm(subdivisions: u8, beat_type: &NoteDuration, dotted: u8, bpm: u32) -> f64 {
        let duration = beat_type.to_ticks(subdivisions) as f64;
        let mut addition = 0.0;
        for i in 1..dotted + 1 {
            let slice = f64::from(2.0).powf(f64::from(i));
            addition = addition + (duration / slice);
        }
        let duration = duration + addition;
        f64::from(bpm) * (duration / f64::from(NoteDuration::Quarter.to_ticks(subdivisions)))
    }

    fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: Spaces(1.0),
            height: Spaces(4.0),
            padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_absolute_tempo(
        &mut self,
        flow_key: &str,
        tick: u32,
        text: &str,
        beat_type: NoteDuration,
        dotted: u8,
        bpm: u32,
        parenthesis_visible: bool,
        text_visible: bool,
        bpm_visible: bool,
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

        // TODO: we want to also find relative tempoes etc.
        // if there is already a absolute tempo at this tick we remove it
        let old_key = match flow.master.get_absolute_tempo_at_tick(tick) {
            Some(tempo) => Some(tempo.key.clone()),
            None => None,
        };

        match old_key {
            Some(key) => {
                flow.master.remove(&key);
            }
            None => (),
        };

        // we insert the new tempo
        let entry = AbsoluteTempo::new(
            flow.subdivisions,
            key.clone(),
            tick,
            String::from(text),
            beat_type,
            dotted,
            bpm,
            parenthesis_visible,
            text_visible,
            bpm_visible,
        );

        // we are now done with the entry, insert it back in
        flow.master.insert(entry);
        self.state
            .ticks
            .insert(String::from(flow_key), flow.calc_ticks());
        self.state.score.meta.set_modified();
        self.emit();

        JsValue::from_str(key.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        // 8th @ 120bpm => 60bpm
        assert_eq!(
            AbsoluteTempo::normalize_bpm(16, &NoteDuration::Eighth, 0, 120),
            60.0
        );
    }

    #[test]
    fn test_two() {
        // half @ 120bpm => 240bpm
        assert_eq!(
            AbsoluteTempo::normalize_bpm(16, &NoteDuration::Half, 0, 120),
            240.0
        );
    }

    #[test]
    fn test_three() {
        // dotted half @ 100bpm => 300bpm
        assert_eq!(
            AbsoluteTempo::normalize_bpm(16, &NoteDuration::Half, 1, 100),
            300.0
        );
    }

    #[test]
    fn test_four() {
        // dotted half @ 120bpm => 60bpm
        assert_eq!(
            AbsoluteTempo::normalize_bpm(16, &NoteDuration::Quarter, 1, 100),
            150.0
        );
    }
}
