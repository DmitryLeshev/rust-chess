use crate::{HORIZONTAL, VERTICAL};

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub name: String,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        let idx_x = x - 1;
        let idx_y = y - 1;
        let name = format!("{}{}", HORIZONTAL[idx_y as usize], VERTICAL[idx_x as usize]);
        Self { x, y, name }
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn create_a_new_position() {
        let x: u8 = 2;
        let y: u8 = 5;
        let name = String::from("e2");
        let right_p = Position::new(x, y);
        let left_p = Position { x, y, name };
        assert_eq!(left_p, right_p);
    }
}
