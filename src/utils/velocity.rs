#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Velocity {
    int: u8,
}

impl Velocity {
    pub fn new(int: u8) -> Self {
        Self { int }
    }
}
