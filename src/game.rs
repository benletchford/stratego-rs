use crate::piece::Piece;
use crate::position::Position;
use crate::rank::Rank;
use crate::team::Team;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    pub board: HashMap<Position, Piece>,
    pub current_player: Team,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: HashMap::new(),
            current_player: Team::Red, // Red usually starts
        }
    }

    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        self.board.insert(position, piece);
    }
}
