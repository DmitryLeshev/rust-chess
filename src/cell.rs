use crate::{figure::Figure, position::Position};

#[derive(Debug, Clone)]
pub struct Cell {
    position: Position,
    figure: Option<Figure>,
}

impl Cell {
    pub fn new(x: u8, y: u8, figure: Option<Figure>) -> Self {
        let position = Position::new(x, y);
        Self { position, figure }
    }
}
