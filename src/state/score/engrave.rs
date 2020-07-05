use crate::state::entries::barline::BarlineType;
use crate::utils::measurements::{Padding, Spaces, MM};
use crate::utils::shortid;
use crate::utils::text::{Font, Justify};

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum Bracketing {
    None,
    Orchestral,
    SmallEnsemble,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum BracketStyle {
    None,
    Wing,
    Line,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum LayoutType {
    Score,
    Part,
    Custom,
}

#[derive(Serialize)]
pub struct Engrave {
    pub key: String,
    pub layout_type: LayoutType,
    pub display_name: String,

    pub space: MM,

    pub frame_padding: Padding<MM>,
    pub instrument_spacing: Spaces,
    pub stave_spacing: Spaces,
    pub system_start_padding: Spaces,

    pub instrument_name: Font,
    pub tempo_text: Font,

    pub systemic_barline_single_instrument_system: bool,
    pub bracketing: Bracketing,
    pub bracket_style: BracketStyle,
    pub bracket_single_staves: bool,
    pub sub_bracket: bool,

    pub minimum_note_spacing: Spaces,

    pub final_barline_type: BarlineType,
}

impl Engrave {
    pub fn new(layout_type: LayoutType, display_name: String) -> Engrave {
        Engrave {
            key: shortid(),
            layout_type,
            display_name,

            space: MM(1.75),

            frame_padding: Padding(MM(40.0), MM(25.0), MM(40.0), MM(25.0)),
            instrument_spacing: Spaces(8.0),
            stave_spacing: Spaces(6.0),
            system_start_padding: Spaces(0.75),

            instrument_name: Font {
                size: Spaces(1.75),
                font: String::from("Libre Baskerville"),
                align: Justify::End,
                padding: Padding(Spaces(0.0), Spaces(2.0), Spaces(0.0), Spaces(0.0)),
            },
            tempo_text: Font {
                size: Spaces(1.75),
                font: String::from("Libre Baskerville"),
                align: Justify::Start,
                padding: Padding(Spaces(0.0), Spaces(0.0), Spaces(2.0), Spaces(0.0)),
            },

            systemic_barline_single_instrument_system: false,
            bracketing: Bracketing::Orchestral,
            bracket_style: BracketStyle::Wing,
            bracket_single_staves: false,
            sub_bracket: true,

            minimum_note_spacing: Spaces(1.6),

            final_barline_type: BarlineType::Final,
        }
    }
}
