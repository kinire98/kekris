use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DangerLevel {
    Empty,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
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
