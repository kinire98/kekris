use crate::{
    game::pieces::Piece,
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        room_info::RoomInfo,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerRoomNetCommands {
    RoomDiscoverResponse(RoomInfo),
    JoinRoomRequestAccepted(DummyRoom),
    JoinRoomRequestRejected(RejectReason),
    PlayersUpdate(Vec<DummyPlayer>),
    RoomClosed(CloseReason),
    PingRequest(bool),
    DisconnectedSignal,
    GameStarts((u64, Vec<Piece>)),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RejectReason {
    RoomFull,
    RoomClosed,
    InnerError,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CloseReason {
    ClosedByHost,
    InnerError,
}
