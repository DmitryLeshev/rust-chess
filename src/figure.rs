use super::position::Position;

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
}

impl Figure {
    pub fn new(name: FigureName, position: Position, color: FigureColor) -> Self {
        Self {
            name,
            position,
            color,
            move_counter: 0,
            kill_counter: 0,
            is_live: true,
        }
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
            is_live: is_live.clone(),
            move_counter: move_counter.clone(),
            kill_counter: kill_counter.clone(),
        };
        let right_f = Figure::new(name, position, color);

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
