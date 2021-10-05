#[derive(Debug)]
pub struct Playground {
    min_x: u8,
    min_y: u8,
    max_x: u8,
    max_y: u8,
}

impl Playground {
    pub fn new() -> Self {
        Self {
            min_x: 1,
            min_y: 1,
            max_x: 8,
            max_y: 8,
        }
    }
}
