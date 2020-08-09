use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u8)]
pub enum Accidental {
    DoubleSharp,
    Sharp,
    Natural,
    Flat,
    DoubleFlat,
}

impl Accidental {
    // TODO: Make this better by working it out within the context of a key
    /// When there is no user defined accidental, we work it out from the pitch
    pub fn default(int: u8) -> Accidental {
        let step = (int - 12) % 12;
        match step {
            0 | 2 | 4 | 5 | 7 | 9 | 10 => Accidental::Natural,
            _ => Accidental::Sharp,
        }
    }

    /// Convert an accidental to a token
    pub fn to_token(&self) -> &str {
        match self {
            Accidental::DoubleSharp => "${double-sharp}",
            Accidental::Sharp => "${sharp}",
            Accidental::Natural => "${natural}",
            Accidental::Flat => "${flat}",
            Accidental::DoubleFlat => "${double-flat}",
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Pitch {
    // the midi number
    pub int: u8,
    // pitch numbers can be written in different ways, this is defined by the user
    pub accidental: Accidental,
}

impl Pitch {
    /// Create a pitch from a MIDI number and accidental
    pub fn new(int: u8, accidental: Accidental) -> Self {
        Self { int, accidental }
    }

    /// gets the base note pitch (natural) for an accidental note
    /// ie 61 (C#) -> 60 (C), 61 (D flat) -> 62 (D)
    fn natural(&self) -> i8 {
        let int = self.int as i8;
        match self.accidental {
            Accidental::DoubleSharp => (int - 2),
            Accidental::Sharp => (int - 1),
            Accidental::Natural => int,
            Accidental::Flat => (int + 1),
            Accidental::DoubleFlat => (int + 2),
        }
    }

    /// Gets the base note letter for a pitch
    /// ie 61 (C#) -> C, 61 (D flat) -> D
    pub fn letter(&self) -> &str {
        const C0: i8 = 12;
        let step = (self.natural() - C0) % 12;

        match step {
            0 => "C",
            2 => "D",
            4 => "E",
            5 => "F",
            7 => "G",
            9 => "A",
            11 => "B",
            _ => "",
        }
    }

    pub fn octave(&self) -> u8 {
        const C0: f32 = 12.0;
        let natural = self.natural() as f32;
        ((natural - C0) / 12.0).floor() as u8
    }

    /// Get the scientific notation parts for the pitch in form (pitch: String, accidental: Accidental, octave: u8)
    /// eg. ("c", Accidental::Sharp, 0) == Pitch(60);
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.letter(),
            self.accidental.to_token(),
            self.octave()
        )
    }

    pub fn to_frequency(&self) -> f64 {
        let a: f64 = 440.0;
        (a / 32.0) * ((2.0 as f64).powf((self.int as f64 - 9.0) / 12.0))
    }
}
