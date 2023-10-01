#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GravityImmune;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Calcium {
    pub amount: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Greenness {
    pub amount: u32,
}
