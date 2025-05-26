use crate::game::{board::danger_level::DangerLevel, pieces::Piece, strategy::Strategy};

use super::dummy_room::DummyPlayer;

/// `OnlineToRemoteGameCommunication` represents the commands that the online game can send to a remote game.
#[derive(Debug, Clone)]
pub enum OnlineToRemoteGameCommunication {
    /// Indicates that trash lines have been received.
    TrashReceived(DummyPlayer, u32),
    /// Represents the queue of upcoming pieces.
    Queue(Vec<Piece>),
    /// Requests the most recent player who sent trash to this player.
    MostRecentReceivedPlayerRequest,
    /// Indicates that the player has won the game.
    Won,
    /// Indicates that a player has lost the game.
    PlayerLost(DummyPlayer),
    /// Indicates that the game has ended.
    GameEnded(DummyPlayer),
    /// Represents the state of another player's board.
    State(DummyPlayer, String),
}
/// `RemoteToOnlineGameCommunication` represents the commands that a remote game can send to the online game.
#[derive(Debug, Clone)]
pub enum RemoteToOnlineGameCommunication {
    /// Indicates that trash lines have been sent.
    TrashSent(DummyPlayer, Strategy, u32),
    /// Represents the current state of the game board.
    BoardState(DummyPlayer, String),
    /// Represents the current danger level of the board.
    DangerLevel(DummyPlayer, DangerLevel),
    /// Sends the player that most recently sent trash.
    HighestReceivedPlayer(DummyPlayer, Option<DummyPlayer>),
    /// Indicates that the player has lost the game.
    Lost(DummyPlayer),
    /// Requests the current queue of upcoming pieces.
    QueueRequest,
}
