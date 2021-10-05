#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn intersects(&self, position: &Position) -> bool {
        self == position
    }
}
