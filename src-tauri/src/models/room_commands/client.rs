use serde::{Deserialize, Serialize};

use crate::models::dummy_room::DummyPlayer;

/// Commands send by the client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRoomNetCommands {
    RoomDiscover,
    JoinRoomRequest(DummyPlayer),
    LeaveRoom(DummyPlayer),
}
