use super::position::Position;

#[derive(Debug, Clone)]
pub enum FigureColor {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub enum FigureName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
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
