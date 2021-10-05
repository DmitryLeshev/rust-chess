mod figure;
mod playground;
mod position;

use figure::{Figure, FigureColor, FigureName};
use position::Position;
fn main() {
    let figure = Figure::new(FigureName::Pawn, Position::new(2, 5), FigureColor::White);
    let figure = figure.move_figure(Position::new(4, 5));
    figure.info();
}
