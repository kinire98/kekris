use crate::{
    game::pieces::Piece,
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        game_options::GameOptions,
        room_info::RoomInfo,
    },
};
use serde::{Deserialize, Serialize};

/// `ServerRoomNetCommands` represents the commands that the server can send to clients to manage the room.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerRoomNetCommands {
    /// Response to a room discovery request.
    /// Contains the `RoomInfo` for the discovered room.
    RoomDiscoverResponse(RoomInfo),
    /// Indicates that a join room request has been accepted.
    /// Contains the `DummyRoom` information for the room the player has joined.
    JoinRoomRequestAccepted(DummyRoom),
    /// Indicates that a join room request has been rejected.
    /// Contains the `RejectReason` for the rejection.
    JoinRoomRequestRejected(RejectReason),
    /// Sends an update of the players in the room.
    /// Contains a vector of `DummyPlayer` representing the players in the room.
    PlayersUpdate(Vec<DummyPlayer>),
    /// Indicates that the room has been closed.
    /// Contains the `CloseReason` for the closure.
    RoomClosed(CloseReason),
    /// Requests a ping response from the client.
    /// The boolean value is not used.
    PingRequest(bool),
    /// Indicates that the server is about to disconnect the client.
    /// The boolean value is not used.
    DisconnectedSignal(bool),
    /// Indicates that the game is starting.
    /// Contains the game start delay, the pieces, the game options, and the local player id.
    GameStarts((u64, Vec<Piece>, GameOptions, u16)),
}
/// `RejectReason` represents the reasons why a join room request can be rejected.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RejectReason {
    /// The room is full.
    RoomFull,
    /// The room is closed.
    RoomClosed,
    /// An internal error occurred.
    InnerError,
}
/// `CloseReason` represents the reasons why a room can be closed.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CloseReason {
    /// The room was closed by the host.
    ClosedByHost,
    /// An internal error occurred.
    InnerError,
}
