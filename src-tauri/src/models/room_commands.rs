use serde::{Deserialize, Serialize};

use super::room_info::RoomInfo;

#[derive(Serialize, Deserialize, Debug)]
pub enum RoomNetCommands {
    RoomDiscover,
    RoomDiscoverResponse(RoomInfo),
}
