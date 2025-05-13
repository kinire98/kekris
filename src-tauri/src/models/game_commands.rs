use crate::game::{pieces::Piece, strategy::Strategy};

#[derive(Debug)]
pub enum FirstLevelCommands {
    RightMove,
    LeftMove,
    ClockWiseRotation,
    CounterClockWiseRotation,
    HardDrop,
    SoftDrop,
    SavePiece,
    FullRotation,
}
#[derive(Debug)]
pub enum SecondLevelCommands {
    AskForQueue,
    QueueSync(Vec<Piece>),
    TrashReceived(u32),
    StrategyChange(Strategy),
    Won,
}
