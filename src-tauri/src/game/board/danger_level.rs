use serde::{Deserialize, Serialize};

/// `DangerLevel` represents the level of danger on the game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DangerLevel {
    /// Represents an empty or no danger level.
    Empty,
    /// Represents a very low danger level.
    VeryLow,
    /// Represents a low danger level.
    Low,
    /// Represents a medium danger level.
    Medium,
    /// Represents a high danger level.
    High,
    /// Represents a very high danger level.
    VeryHigh,
    /// Represents a danger level where the player is almost dead.
    AlmostDead,
}

#[cfg(test)]
mod test {
    use crate::game::board::danger_level::DangerLevel;

    #[test]
    fn test_order() {
        assert!(DangerLevel::Empty < DangerLevel::VeryLow);
        assert!(DangerLevel::VeryHigh < DangerLevel::AlmostDead);
        assert!(DangerLevel::Empty == DangerLevel::Empty);
        assert!(DangerLevel::High > DangerLevel::Medium);
    }
}
