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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingSystem {
    UsEu,
    ClassicUs,
}

impl Rank {
    pub fn value(&self, system: RankingSystem) -> Option<u8> {
        match system {
            RankingSystem::UsEu => match self {
                Rank::Flag => None,
                Rank::Bomb => None,
                Rank::Spy => Some(1),
                Rank::Scout => Some(2),
                Rank::Miner => Some(3),
                Rank::Sergeant => Some(4),
                Rank::Lieutenant => Some(5),
                Rank::Captain => Some(6),
                Rank::Major => Some(7),
                Rank::Colonel => Some(8),
                Rank::General => Some(9),
                Rank::Marshal => Some(10),
            },
            RankingSystem::ClassicUs => match self {
                Rank::Flag => None,
                Rank::Bomb => None,
                Rank::Spy => Some(8),
                Rank::Scout => Some(7),
                Rank::Miner => Some(6),
                Rank::Sergeant => Some(5),
                Rank::Lieutenant => Some(4),
                Rank::Captain => Some(3),
                Rank::Major => Some(2),
                Rank::Colonel => Some(1),
                Rank::General => Some(10),
                Rank::Marshal => Some(11),
            },
        }
    }

    /// Determines if this rank can capture another rank
    pub fn can_capture(&self, other: &Rank, system: RankingSystem) -> bool {
        match (self, other) {
            // Special case: Spy can capture Marshal if spy attacks first
            (Rank::Spy, Rank::Marshal) => true,
            // Special case: Only Miners can capture Bombs
            (Rank::Miner, Rank::Bomb) => true,
            // Flag cannot capture
            (Rank::Flag, _) => false,
            // Bombs cannot capture
            (Rank::Bomb, _) => false,
            // Normal case
            _ => {
                if let (Some(v1), Some(v2)) = (self.value(system), other.value(system)) {
                    match system {
                        RankingSystem::ClassicUs => {
                            // In ClassicUs, lower numbers (1-8) are stronger and can capture higher numbers
                            // General (10) and Marshal (11) are actually weaker against regular pieces
                            if v1 <= 8 && v2 <= 8 {
                                // For regular pieces (1-8), lower number wins
                                v1 <= v2
                            } else if v1 <= 8 {
                                // Regular piece vs General/Marshal - regular piece wins
                                true
                            } else {
                                // General/Marshal vs regular piece - regular piece wins
                                false
                            }
                        }
                        RankingSystem::UsEu => v1 >= v2,
                    }
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_cases() {
        // Test Spy vs Marshal special case in both systems
        assert_eq!(
            Rank::Spy.can_capture(&Rank::Marshal, RankingSystem::UsEu),
            true,
            "Spy should capture Marshal in UsEu"
        );
        assert_eq!(
            Rank::Spy.can_capture(&Rank::Marshal, RankingSystem::ClassicUs),
            true,
            "Spy should capture Marshal in ClassicUs"
        );

        // Test Miner vs Bomb special case in both systems
        assert_eq!(
            Rank::Miner.can_capture(&Rank::Bomb, RankingSystem::UsEu),
            true,
            "Miner should capture Bomb in UsEu"
        );
        assert_eq!(
            Rank::Miner.can_capture(&Rank::Bomb, RankingSystem::ClassicUs),
            true,
            "Miner should capture Bomb in ClassicUs"
        );

        // Test Flag cannot capture in both systems
        assert_eq!(
            Rank::Flag.can_capture(&Rank::Scout, RankingSystem::UsEu),
            false,
            "Flag cannot capture in UsEu"
        );
        assert_eq!(
            Rank::Flag.can_capture(&Rank::Scout, RankingSystem::ClassicUs),
            false,
            "Flag cannot capture in ClassicUs"
        );

        // Test Bomb cannot capture in both systems
        assert_eq!(
            Rank::Bomb.can_capture(&Rank::Scout, RankingSystem::UsEu),
            false,
            "Bomb cannot capture in UsEu"
        );
        assert_eq!(
            Rank::Bomb.can_capture(&Rank::Scout, RankingSystem::ClassicUs),
            false,
            "Bomb cannot capture in ClassicUs"
        );
    }

    #[test]
    fn test_invalid_captures() {
        // Test cannot capture Bomb (except Miner)
        assert_eq!(
            Rank::Marshal.can_capture(&Rank::Bomb, RankingSystem::UsEu),
            false,
            "Marshal cannot capture Bomb in UsEu"
        );
        assert_eq!(
            Rank::Scout.can_capture(&Rank::Bomb, RankingSystem::ClassicUs),
            false,
            "Scout cannot capture Bomb in ClassicUs"
        );

        // Test cannot capture Flag
        assert_eq!(
            Rank::Marshal.can_capture(&Rank::Flag, RankingSystem::UsEu),
            false,
            "Marshal cannot capture Flag in UsEu"
        );
        assert_eq!(
            Rank::Scout.can_capture(&Rank::Flag, RankingSystem::ClassicUs),
            false,
            "Scout cannot capture Flag in ClassicUs"
        );
    }

    #[test]
    fn test_equal_rank_captures() {
        // In UsEu, equal ranks can capture each other
        assert_eq!(
            Rank::Scout.can_capture(&Rank::Scout, RankingSystem::UsEu),
            true,
            "Equal ranks can capture in UsEu"
        );
        assert_eq!(
            Rank::Captain.can_capture(&Rank::Captain, RankingSystem::UsEu),
            true,
            "Equal ranks can capture in UsEu"
        );

        // In ClassicUs, equal ranks can capture each other (for regular pieces)
        assert_eq!(
            Rank::Scout.can_capture(&Rank::Scout, RankingSystem::ClassicUs),
            true,
            "Equal regular ranks can capture in ClassicUs"
        );
        assert_eq!(
            Rank::Captain.can_capture(&Rank::Captain, RankingSystem::ClassicUs),
            true,
            "Equal regular ranks can capture in ClassicUs"
        );
    }

    #[test]
    fn test_us_eu_regular_captures() {
        // Higher ranks capture lower ranks
        assert_eq!(
            Rank::Marshal.can_capture(&Rank::General, RankingSystem::UsEu),
            true,
            "Marshal should capture General"
        );
        assert_eq!(
            Rank::General.can_capture(&Rank::Colonel, RankingSystem::UsEu),
            true,
            "General should capture Colonel"
        );
        assert_eq!(
            Rank::Colonel.can_capture(&Rank::Major, RankingSystem::UsEu),
            true,
            "Colonel should capture Major"
        );

        // Lower ranks cannot capture higher ranks
        assert_eq!(
            Rank::Scout.can_capture(&Rank::Marshal, RankingSystem::UsEu),
            false,
            "Scout cannot capture Marshal"
        );
        assert_eq!(
            Rank::Sergeant.can_capture(&Rank::Colonel, RankingSystem::UsEu),
            false,
            "Sergeant cannot capture Colonel"
        );
    }

    #[test]
    fn test_classic_us_regular_captures() {
        // Test regular piece hierarchy (1-8)
        assert_eq!(
            Rank::Colonel.can_capture(&Rank::Major, RankingSystem::ClassicUs),
            true,
            "Colonel (1) should capture Major (2)"
        );
        assert_eq!(
            Rank::Major.can_capture(&Rank::Captain, RankingSystem::ClassicUs),
            true,
            "Major (2) should capture Captain (3)"
        );
        assert_eq!(
            Rank::Captain.can_capture(&Rank::Lieutenant, RankingSystem::ClassicUs),
            true,
            "Captain (3) should capture Lieutenant (4)"
        );

        // Test regular pieces vs General/Marshal
        assert_eq!(
            Rank::Colonel.can_capture(&Rank::General, RankingSystem::ClassicUs),
            true,
            "Colonel should capture General"
        );
        assert_eq!(
            Rank::Major.can_capture(&Rank::Marshal, RankingSystem::ClassicUs),
            true,
            "Major should capture Marshal"
        );

        // Test General/Marshal cannot capture regular pieces
        assert_eq!(
            Rank::General.can_capture(&Rank::Colonel, RankingSystem::ClassicUs),
            false,
            "General cannot capture Colonel"
        );
        assert_eq!(
            Rank::Marshal.can_capture(&Rank::Major, RankingSystem::ClassicUs),
            false,
            "Marshal cannot capture Major"
        );
    }
}
