use serde::{Deserialize, Serialize};

use crate::game::{board::danger_level::DangerLevel, strategy::Strategy};

/// `ClientOnlineGameCommands` represents the commands that a client can send to the server during an online game.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ClientOnlineGameCommands {
    /// Indicates that the client has sent trash lines to other players.
    /// Contains the strategy used and the amount of trash sent.
    TrashSent(Strategy, u32),
    /// Represents the current state of the client's game board as a string.
    BoardState(String),
    /// Indicates the client's current danger level.
    DangerLevel(DangerLevel),
    /// Indicates that the client has lost the game.
    /// The `u8` value serves no purpose but is included for serialization/deserialization.
    Lost(u8), // Those number serve no purpose but have content to serialize and deserialize
    /// Requests the current queue of upcoming pieces from the server.
    /// The `u8` value serves no purpose but is included for serialization/deserialization.
    QueueRequest(u8), // Those number serve no purpose but have content to serialize and deserialize
}
