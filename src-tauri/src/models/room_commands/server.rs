use crate::models::{
    dummy_room::{DummyPlayer, DummyRoom},
    room_info::RoomInfo,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerRoomNetCommands {
    RoomDiscoverResponse(RoomInfo),
    JoinRoomRequestAccepted(DummyRoom),
    JoinRoomRequestRejected(RejectReason),
    PlayersUpdate(Vec<DummyPlayer>),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RejectReason {
    RoomFull,
    RoomClosed,
    InnerError,
}
