use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize_repr, Deserialize_repr, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Expression {
    Natural,
    Pizzicato,
    Spiccato,
    Staccato,
    Tremolo,
    Mute, // may have to convert to MuteStaccato etc. later for mixed expressions
}

#[derive(Serialize, Deserialize)]
pub struct StaveDef {
    pub lines: u8,
    pub clef_pitch: u8, // these are the default clef for the instrument track
    pub clef_offset: u8,
}

impl StaveDef {
    pub fn new(lines: u8, clef_pitch: u8, clef_offset: u8) -> Self {
        Self {
            lines,
            clef_pitch,
            clef_offset: clef_offset,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InstrumentDef {
    pub id: &'static str,
    pub path: Vec<&'static str>,
    pub long_name: &'static str,
    pub short_name: &'static str,
    pub staves: Vec<StaveDef>,
    pub patches: HashMap<Expression, &'static str>,
}

lazy_static! {
    pub static ref INSTRUMENT_DEFS: Vec<InstrumentDef> = {
        vec![
            InstrumentDef {
                id: "brass.bass-trombone",
                path: vec!["Brass", "Bass Trombone"],
                long_name: "Bass Trombone",
                short_name: "B. Tbn.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/bass-trombone/natural.json"
                },
            },
            InstrumentDef {
                id: "brass.horn.f",
                path: vec!["Brass", "Horn", "F"],
                long_name: "Horn in F",
                short_name: "F Hn.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/horn/natural.json",
                    Expression::Staccato => "/patches/horn/staccato.json",
                    Expression::Mute => "/patches/horn/mute.json",
                },
            },
            InstrumentDef {
                id: "brass.trombone",
                path: vec!["Brass", "Trombone"],
                long_name: "Trombone",
                short_name: "Tbn.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/trombone/natural.json",
                    Expression::Staccato => "/patches/trombone/staccato.json"
                },
            },
            InstrumentDef {
                id: "brass.trumpet.b-flat",
                path: vec!["Brass", "Trumpet", "B${flat}"],
                long_name: "Trumpet in B${flat}",
                short_name: "B${flat} Tpt.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/trumpet/natural.json",
                    Expression::Staccato => "/patches/trumpet/staccato.json",
                    Expression::Mute => "/patches/trumpet/mute.json"
                },
            },
            InstrumentDef {
                id: "brass.trumpet.c",
                path: vec!["Brass", "Trumpet", "C"],
                long_name: "Trumpet in C",
                short_name: "C Tpt.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/trumpet/natural.json",
                    Expression::Staccato => "/patches/trumpet/staccato.json",
                    Expression::Mute => "/patches/trumpet/mute.json"
                },
            },
            InstrumentDef {
                id: "brass.tuba",
                path: vec!["Brass", "Tuba"],
                long_name: "Tuba",
                short_name: "Tba.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/tuba/natural.json",
                    Expression::Staccato => "/patches/tuba/staccato.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.glockenspiel",
                path: vec!["Pitched Percussion", "Glockenspiel"],
                long_name: "Glokenspiel",
                short_name: "Glock.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/glockenspiel/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.harp",
                path: vec!["Pitched Percussion", "Harp"],
                long_name: "Harp",
                short_name: "Hrp.",
                staves: vec![StaveDef::new(5, 67, 6), StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/harp/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.marimba",
                path: vec!["Pitched Percussion", "Marimba"],
                long_name: "Marimba",
                short_name: "Mrm.",
                staves: vec![StaveDef::new(5, 67, 6), StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/marimba/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.timpani",
                path: vec!["Pitched Percussion", "Timpani"],
                long_name: "Timpani",
                short_name: "Timp.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/timpani/natural.json",
                    Expression::Tremolo => "/patches/timpani/roll.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.vibraphone",
                path: vec!["Pitched Percussion", "Vibraphone"],
                long_name: "Vibraphone",
                short_name: "Vib.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/vibraphone/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.xylophone",
                path: vec!["Pitched Percussion", "Xylophone"],
                long_name: "Xylophone",
                short_name: "Xyl.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/xylophone/natural.json"
                },
            },
            InstrumentDef {
                id: "keyboard.celesta",
                path: vec!["Keyboards", "Celesta"],
                long_name: "Celesta",
                short_name: "Cel.",
                staves: vec![StaveDef::new(5, 67, 6), StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/celesta/natural.json"
                },
            },
            InstrumentDef {
                id: "keyboard.piano",
                path: vec!["Keyboards", "Piano"],
                long_name: "Piano",
                short_name: "Pno.",
                staves: vec![StaveDef::new(5, 67, 6), StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/piano/natural.json"
                },
            },
            InstrumentDef {
                id: "strings.contrabass",
                path: vec!["Strings", "Contrabass"],
                long_name: "Contrabass",
                short_name: "Cb.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/contrabass/natural.json",
                    Expression::Pizzicato => "/patches/contrabass/pizzicato.json",
                    Expression::Staccato => "/patches/contrabass/spiccato.json"
                },
            },
            InstrumentDef {
                id: "strings.viola",
                path: vec!["Strings", "Viola"],
                long_name: "Viola",
                short_name: "Vla.",
                staves: vec![StaveDef::new(5, 60, 4)],
                patches: hashmap! {
                    Expression::Natural => "/patches/viola/natural.json",
                    Expression::Pizzicato => "/patches/viola/pizzicato.json",
                    Expression::Staccato => "/patches/viola/staccato.json"
                },
            },
            InstrumentDef {
                id: "strings.violin",
                path: vec!["Strings", "Violin"],
                long_name: "Violin",
                short_name: "Vln.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/violin/natural.json",
                    Expression::Pizzicato => "/patches/violin/pizzicato.json",
                    Expression::Staccato => "/patches/violin/spiccato.json",
                    Expression::Tremolo => "/patches/violin/tremolo.json"
                },
            },
            InstrumentDef {
                id: "strings.violoncello",
                path: vec!["Strings", "Violoncello"],
                long_name: "Violoncello",
                short_name: "Vc.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/violoncello/natural.json",
                    Expression::Pizzicato => "/patches/violoncello/pizzicato.json",
                    Expression::Staccato => "/patches/violoncello/staccato.json"
                },
            },
            InstrumentDef {
                id: "woodwinds.alto-flute",
                path: vec!["Woodwinds", "Alto Flute"],
                long_name: "Alto Flute",
                short_name: "A. Fl.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/alto-flute/natural.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.bassoon",
                path: vec!["Woodwinds", "Bassoon"],
                long_name: "Bassoon",
                short_name: "Bsn.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/bassoon/natural.json",
                    Expression::Staccato => "/patches/bassoon/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.bass-clarinet",
                path: vec!["Woodwinds", "Bass Clarinet"],
                long_name: "Bass Clarinet",
                short_name: "B. Cl.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/bass-clarinet/natural.json",
                    Expression::Staccato => "/patches/bass-clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.clarinet.a",
                path: vec!["Woodwinds", "Clarinet", "A"],
                long_name: "Clarinet in A",
                short_name: "A Cl.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.clarinet.b-flat",
                path: vec!["Woodwinds", "Clarinet", "B Flat"],
                long_name: "Clarinet in B${flat}",
                short_name: "B${flat} Cl.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.contrabassoon",
                path: vec!["Woodwinds", "Contrabassoon"],
                long_name: "Contrabasson",
                short_name: "Cbsn.",
                staves: vec![StaveDef::new(5, 53, 2)],
                patches: hashmap! {
                    Expression::Natural => "/patches/contrabassoon/natural.json"
                },
            },
            InstrumentDef {
                id: "woodwinds.english-horn",
                path: vec!["Woodwinds", "English Horn"],
                long_name: "English Horn",
                short_name: "E Hn.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/cor-anglais/natural.json",
                    Expression::Staccato => "/patches/cor-anglais/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.flute",
                path: vec!["Woodwinds", "Flute"],
                long_name: "Flute",
                short_name: "Fl.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/flute/natural.json",
                    Expression::Staccato => "/patches/flute/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.oboe",
                path: vec!["Woodwinds", "Oboe"],
                long_name: "Oboe",
                short_name: "Ob.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/oboe/natural.json",
                    Expression::Staccato => "/patches/oboe/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.piccolo",
                path: vec!["Woodwinds", "Piccolo"],
                long_name: "Piccolo",
                short_name: "Pc.",
                staves: vec![StaveDef::new(5, 67, 6)],
                patches: hashmap! {
                    Expression::Natural => "/patches/piccolo/natural.json",
                    Expression::Staccato => "/patches/piccolo/staccato.json",
                },
            },
        ]
    };
}

pub fn get_def(id: &str) -> Option<&InstrumentDef> {
    INSTRUMENT_DEFS.iter().find(|&def| def.id == id)
}

/// Get patches for a given id
#[wasm_bindgen]
pub fn get_patches(id: &str) -> JsValue {
    let def = match get_def(id) {
        Some(def) => def,
        None => return JsValue::UNDEFINED,
    };

    JsValue::from_serde(&def.patches).unwrap()
}
