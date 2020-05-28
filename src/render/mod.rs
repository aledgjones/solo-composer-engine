pub mod text;
use text::TextStyles;

#[derive(Serialize)]
pub enum Instruction {
    Text {
        key: String,
        identifier: String,
        styles: TextStyles,
        value: String,
        x: u32,
        y: u32,
    },
}

#[derive(Serialize)]
pub struct Instructions {
    pub space: u32,
    pub height: u32,
    pub width: u32,
    pub entries: Vec<Instruction>,
}
