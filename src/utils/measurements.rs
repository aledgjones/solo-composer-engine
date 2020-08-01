#[derive(Serialize, Deserialize)]
pub struct MM(pub f32);

#[derive(Serialize, Deserialize)]
pub struct Spaces(pub f32);

#[derive(Serialize, Deserialize)]
pub struct BoundingBox {
    pub width: Spaces,
    pub height: Spaces,
    pub padding: Padding<Spaces>,
}

#[derive(Serialize, Deserialize)]
pub struct Padding<T>(pub T, pub T, pub T, pub T);
