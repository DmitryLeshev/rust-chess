#[derive(Debug)]
pub struct Playground {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
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
