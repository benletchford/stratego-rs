use crate::rank::Rank;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub rank: Rank,
    pub revealed: bool,
}

impl Piece {
    pub fn new(rank: Rank) -> Self {
        Piece {
            rank,
            revealed: false,
        }
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }
}
