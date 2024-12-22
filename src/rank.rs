#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Flag,
    Spy,
    Scout,
    Miner,
    Sergeant,
    Lieutenant,
    Captain,
    Major,
    Colonel,
    General,
    Marshal,
    Bomb,
}

impl Rank {
    /// Returns the numeric value of the rank for comparison
    pub fn value(&self) -> u8 {
        match self {
            Rank::Flag => 0,
            Rank::Spy => 1,
            Rank::Scout => 2,
            Rank::Miner => 3,
            Rank::Sergeant => 4,
            Rank::Lieutenant => 5,
            Rank::Captain => 6,
            Rank::Major => 7,
            Rank::Colonel => 8,
            Rank::General => 9,
            Rank::Marshal => 10,
            Rank::Bomb => 11,
        }
    }

    /// Determines if this rank can capture another rank
    pub fn can_capture(&self, other: &Rank) -> bool {
        match (self, other) {
            // Special case: Spy can capture Marshal if spy attacks first
            (Rank::Spy, Rank::Marshal) => true,
            // Special case: Only Miners can capture Bombs
            (_, Rank::Bomb) => *self == Rank::Miner,
            // Flag cannot capture or be captured (it ends the game when found)
            (Rank::Flag, _) | (_, Rank::Flag) => false,
            // Bombs cannot move or capture
            (Rank::Bomb, _) => false,
            // Normal case: Higher rank captures lower rank
            _ => self.value() >= other.value(),
        }
    }
}
