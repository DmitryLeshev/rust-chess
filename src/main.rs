pub const VERTICAL: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
pub const HORIZONTAL: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

mod board;
mod cell;
mod figure;
mod game;
mod playground;
mod position;

use board::Board;
use figure::{Figure, FigureColor, FigureName};
use position::Position;
fn main() {
    let board = Board::new();

    let cell_e2 = board.cells.get("e2").unwrap();
    println!("cell_e2 {:?} \n", cell_e2);
    if let Some(f) = &cell_e2.figure {
        let white_f = f.clone().figure_movement_analyzer(&board);
        if let Some(white_f_moves) = &white_f.moves {
            let last_cell = white_f_moves.clone().pop().unwrap();
            let white_f = white_f.move_figure(last_cell.position);
            println!("white_pawn {:?} \n", white_f);
        }
    }

    let cell_d7 = board.cells.get("d7").unwrap();
    println!("cell_d7 {:?} \n", cell_d7);
    if let Some(f) = &cell_d7.figure {
        let black_f = f.clone().figure_movement_analyzer(&board);
        if let Some(black_f_moves) = &black_f.moves {
            let last_cell = black_f_moves.clone().pop().unwrap();
            let black_f = black_f.move_figure(last_cell.position);
            println!("black_pawn {:?} \n", black_f);
        }
    }

    let cell_e4 = board.cells.get("e4").unwrap();
    println!("cell_e4 {:?} \n", cell_e4);
    if let Some(f) = &cell_e4.figure {
        let white_f = f.clone().figure_movement_analyzer(&board);
        if let Some(white_f_moves) = &white_f.moves {
            // let last_cell = white_f_moves.clone().pop().unwrap();
            // let white_f = white_f.move_figure(last_cell.position);
            println!("white_pawn {:?} \n", white_f);
        }
    }
}
