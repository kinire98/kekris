use crate::game::{pieces::Piece, strategy::Strategy};

/// `FirstLevelCommands` represents the commands that can be directly triggered by player input.
#[derive(Debug)]
pub enum FirstLevelCommands {
    /// Moves the current piece to the right.
    RightMove,
    /// Moves the current piece to the left.
    LeftMove,
    /// Rotates the current piece clockwise.
    ClockWiseRotation,
    /// Rotates the current piece counter-clockwise.
    CounterClockWiseRotation,
    /// Instantly drops the current piece to the bottom of the board.
    HardDrop,
    /// Moves the current piece down by one row.
    SoftDrop,
    /// Saves the current piece for later use.
    SavePiece,
    /// Rotates the current piece 180 degrees.
    FullRotation,
}
/// `SecondLevelCommands` represents the commands that are triggered by game logic or network events.
#[derive(Debug)]
pub enum SecondLevelCommands {
    /// Asks for the queue of upcoming pieces.
    AskForQueue,
    /// Synchronizes the queue of upcoming pieces.
    QueueSync(Vec<Piece>),
    /// Indicates that trash lines have been received.
    TrashReceived(u32),
    /// Requests a change in the player's strategy.
    StrategyChange(Strategy),
    /// Indicates that the player has won the game.
    Won,
}
