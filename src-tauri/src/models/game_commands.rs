use crate::game::strategy::Strategy;

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
    QueueSync,
    TrashReceived(u8),
    StrategyChange(Strategy),
}
