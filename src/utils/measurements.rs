#[derive(Serialize)]
pub struct MM(pub f32);

#[derive(Serialize)]
pub struct Spaces(pub f32);

#[derive(Serialize)]
pub struct BoundingBox {
    pub width: Spaces,
    pub height: Spaces,
    pub padding: Padding<Spaces>,
}

#[derive(Serialize)]
pub struct Padding<T>(pub T, pub T, pub T, pub T);
