use crate::{
    figure::{Figure, FigureColor, FigureName},
    position::Position,
    HORIZONTAL, VERTICAL,
};

use std::collections::HashMap;

use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: HashMap<String, Cell>,
}

impl Board {
    pub fn new() -> Self {
        let mut cells = HashMap::new();
        for (idx_y, char) in HORIZONTAL.iter().enumerate() {
            let idx_y = idx_y + 1;
            for (idx_x, value) in VERTICAL.iter().enumerate() {
                let idx_x = idx_x + 1;
                let figure_name = match (idx_x, idx_y) {
                    (1 | 8, 1 | 8) => Some(FigureName::Rook),
                    (1 | 8, 2 | 7) => Some(FigureName::Knight),
                    (1 | 8, 3 | 6) => Some(FigureName::Bishop),
                    (1 | 8, 5) => Some(FigureName::King),
                    (1 | 8, 4) => Some(FigureName::Queen),
                    (2 | 7, _) => Some(FigureName::Pawn),
                    _ => None,
                };
                let figure_color = match idx_x {
                    1 | 2 => Some(FigureColor::White),
                    8 | 7 => Some(FigureColor::Black),
                    _ => None,
                };
                let figure = match (figure_name, figure_color) {
                    (Some(n), Some(c)) => {
                        let p = Position::new(idx_x as u8, idx_y as u8);
                        let f = Figure::new(n, p, c);
                        Some(f)
                    }
                    _ => None,
                };
                println!("idx_x: {}  idx_y: {}", idx_x, idx_y);
                let cell = Cell::new(idx_x as u8, idx_y as u8, figure);
                let name = format!("{}{}", char, value);
                cells.insert(name, cell);
            }
        }
        Self { cells }
    }
}
