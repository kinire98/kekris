#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DangerLevel {
    Empty,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    AlmostDead
}