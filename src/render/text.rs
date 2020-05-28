#[derive(Serialize)]
pub enum TextJustify {
    Start,
    Middle,
    End,
}

#[derive(Serialize)]
pub enum TextAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize)]
pub struct TextStyles {
    pub color: String,
    pub font: String,
    pub size: u32,
    pub justify: TextJustify,
    pub align: TextAlign,
}
