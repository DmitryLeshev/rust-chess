use crate::{board::Board, cell::Cell, HORIZONTAL};

use super::position::Position;

type Moves = Option<Vec<Cell>>;

#[derive(Debug, Clone, PartialEq)]
pub enum FigureColor {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FigureName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    pub name: FigureName,
    pub position: Position,
    pub color: FigureColor,
    pub is_live: bool,
    pub move_counter: u32,
    pub kill_counter: u32,
    pub moves: Moves,
}

impl Figure {
    pub fn new(name: FigureName, position: Position, color: FigureColor) -> Self {
        let moves: Moves = None;
        Self {
            name,
            position,
            color,
            move_counter: 0,
            kill_counter: 0,
            is_live: true,
            moves,
        }
    }

    // pub fn new(name: FigureName, position: Position, color: FigureColor, board: &Board) -> Self {
    //     let moves: Moves = None;
    //     Self {
    //         name,
    //         position,
    //         color,
    //         move_counter: 0,
    //         kill_counter: 0,
    //         is_live: true,
    //         moves,
    //     }
    // }

    // fn pawn_move(position: &Position, color: &FigureColor) -> Moves {
    //     let moves: Moves = None;
    //     moves
    // }

    pub fn figure_movement_analyzer(self, board: &Board) -> Self {
        let moves: Moves = match self.name {
            FigureName::Pawn => {
                let check_positions = {
                    let Position { x, y, name } = &self.position;
                    let mut positions = Vec::new();
                    let number_of_moves: u8 = {
                        if self.move_counter > 0 {
                            1
                        } else {
                            2
                        }
                    };
                    for vertical in 0..number_of_moves {
                        let check_x = if self.color == FigureColor::White {
                            x.clone() + vertical + 1
                        } else {
                            x.clone() - vertical - 1
                        };
                        let check_y = y.clone();
                        let p = Position::new(check_x, check_y);
                        positions.push(p);

                        if check_x == x + 1 {
                            let p_left = Position::new(check_x, y - 1);
                            if let Some(cell_left) = board.cells.get(&p_left.name) {
                                if let Some(f) = &cell_left.figure {
                                    if f.color != self.color {
                                        positions.push(p_left);
                                    }
                                }
                            }

                            let p_right = Position::new(check_x, y + 1);
                            if let Some(cell_right) = board.cells.get(&p_right.name) {
                                if let Some(f) = &cell_right.figure {
                                    if f.color != self.color {
                                        positions.push(p_right);
                                    }
                                }
                            }
                        }
                    }
                    positions
                };

                let mut moves = Vec::new();
                for p in check_positions {
                    if let Some(cell) = board.cells.get(&p.name) {
                        let Cell { figure, .. } = cell;
                        match figure {
                            Some(f) => println!("{:?}", f),
                            None => moves.push(cell.clone()),
                        };
                    };
                }

                match moves.len() > 0 {
                    true => Some(moves),
                    _ => None,
                }
            }
            _ => None,
        };
        Self { moves, ..self }
    }

    pub fn info(&self) {
        println!(
            "[{:?}] position: {:?}, move_counter:{:?}",
            self.name, self.position, self.move_counter
        );
    }

    pub fn move_figure(self, position: Position) -> Self {
        self.info();
        Self {
            position,
            move_counter: self.move_counter + 1,
            ..self
        }
    }

    pub fn kill(self, figure: &mut Figure) -> Self {
        figure.die();
        let new_figure = self.move_figure(figure.position.clone());

        Self {
            kill_counter: new_figure.kill_counter + 1,
            ..new_figure
        }
    }

    pub fn die(&mut self) {
        self.is_live = false;
    }
}

mod tests {
    use crate::{
        figure::{Figure, FigureColor, FigureName},
        position::Position,
    };
    #[test]
    fn create_a_new_figure() {
        let x: u8 = 2;
        let y: u8 = 5;
        let p = Position::new(x, y);

        let name = FigureName::King;
        let position = p;
        let color = FigureColor::White;
        let is_live = true;
        let move_counter = 0;
        let kill_counter = 0;

        let left_f = Figure {
            name: name.clone(),
            position: position.clone(),
            color: color.clone(),
            is_live: is_live,
            move_counter: move_counter,
            kill_counter: kill_counter,
            moves: None,
        };
        let right_f = Figure::new(name, position, color);

        assert_eq!(left_f, right_f);
    }
    #[test]
    fn move_a_pawn() {
        let x: u8 = 2;
        let y: u8 = 5;
        let init_position = Position::new(x, y);
        let new_position = Position::new(x + 2, y);

        let name = FigureName::Pawn;
        let color = FigureColor::White;
        let is_live = true;
        let move_counter = 1;
        let kill_counter = 0;

        let left_f = Figure {
            name: name.clone(),
            position: new_position.clone(),
            color: color.clone(),
            is_live: is_live,
            move_counter: move_counter,
            kill_counter: kill_counter,
            moves: None,
        };
        let right_f = Figure::new(name, init_position, color);

        let right_f = right_f.move_figure(new_position);

        assert_eq!(left_f, right_f);
    }
}

// pub fn intersects(&self, figure: &Figure) -> bool {
//     self.position == figure.position
// }

// let new_position = match self.name {
//     FigureName::Pawn => self.move_pawn(position),
//     FigureName::Knight => println!("I'm Knight"),
//     FigureName::Bishop => println!("I'm Bishop"),
//     FigureName::Rook => println!("I'm Rook"),
//     FigureName::Queen => println!("I'm Queen"),
//     FigureName::King => println!("I'm King"),
// }
