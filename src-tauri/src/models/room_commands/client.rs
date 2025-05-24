use serde::{Deserialize, Serialize};

use crate::models::dummy_room::DummyPlayer;

/// Commands send by the client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRoomNetCommands {
    RoomDiscover(bool),
    JoinRoomRequest(DummyPlayer),
    LeaveRoom(DummyPlayer),
    PingResponse(bool),
}
