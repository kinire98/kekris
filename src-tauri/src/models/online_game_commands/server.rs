use serde::{Deserialize, Serialize};

use crate::{game::pieces::Piece, models::dummy_room::DummyPlayer};

/// `ServerOnlineGameCommands` represents the commands that the server can send to clients during an online game.
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerOnlineGameCommands {
    /// Indicates that the server is sending trash lines to the client.
    /// Contains the amount of trash lines being sent.
    TrashSent(u32),
    /// Represents the queue of upcoming pieces.
    Queue(Vec<Piece>),
    /// Indicates that the client has won the game.
    /// The `u8` value serves no purpose but is included for serialization/deserialization.
    Won(u8),
    /// Indicates that a player has lost the game.
    /// Contains the `DummyPlayer` representing the player who lost.
    PlayerLost(DummyPlayer),
    /// Indicates that the game has ended.
    /// Contains the `DummyPlayer` representing the player who won.
    GameEnded(DummyPlayer),
    /// Represents the state of another player's board.
    /// Contains the `DummyPlayer` and the board state as a string.
    State(DummyPlayer, String),
}
