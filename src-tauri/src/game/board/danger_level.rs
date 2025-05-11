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
