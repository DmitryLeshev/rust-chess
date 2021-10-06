pub const VERTICAL: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
pub const HORIZONTAL: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

mod board;
mod cell;
mod figure;
mod playground;
mod position;

use board::Board;
use figure::{Figure, FigureColor, FigureName};
use position::Position;
fn main() {
    let p = Position::new(2, 5);
    println!("p: {:?}", p);
    let board = Board::new();
    println!("{:?}", board);
    // for (key, val) in board.cells.iter() {
    //     println!("key: {:?} val: {:?}", key, val);
    // }
}
