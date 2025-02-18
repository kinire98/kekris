#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strategy {
    Elimination,
    Even,
    PayBack,
    Random
}