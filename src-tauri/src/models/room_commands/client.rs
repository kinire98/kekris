use serde::{Deserialize, Serialize};

use crate::models::dummy_room::DummyPlayer;

/// `ClientRoomNetCommands` represents the commands that a client can send to the server to interact with a room.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRoomNetCommands {
    /// Requests the discovery of available rooms.
    /// The boolean value is not used.
    RoomDiscover(bool),
    /// Requests to join a specific room.
    /// Contains the `DummyPlayer` information for the player joining the room.
    JoinRoomRequest(DummyPlayer),
    /// Requests to leave the current room.
    /// Contains the `DummyPlayer` information for the player leaving the room.
    LeaveRoom(DummyPlayer),
    /// Responds to a ping request from the server.
    /// The boolean value is not used.
    PingResponse(bool),
}
