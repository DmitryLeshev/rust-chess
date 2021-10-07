use std::time::SystemTime;

use crate::board::Board;

enum Color {
    White,
    Black,
}
struct Player {
    color: Color,
    name: Option<String>,
    points: u8,
}

struct Game {
    player_1: Player,
    player_2: Player,
    board: Board,
    player_turn: Player,
    date: SystemTime,
}
