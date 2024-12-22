use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl Position {
    pub fn new(row: u8, col: u8) -> Self {
        Position { row, col }
    }

    /// Returns true if the position is within the 10x10 board
    pub fn is_valid(&self) -> bool {
        self.row < 10 && self.col < 10
    }

    /// Returns true if the position is in a lake (invalid move position)
    pub fn is_lake(&self) -> bool {
        matches!(
            (self.row, self.col),
            (4, 2) | (4, 3) | (5, 2) | (5, 3) | (4, 6) | (4, 7) | (5, 6) | (5, 7)
        )
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}
