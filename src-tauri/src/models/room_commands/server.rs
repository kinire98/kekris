use crate::{
    game::pieces::Piece,
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        game_options::GameOptions,
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
    DisconnectedSignal(bool),
    GameStarts((u64, Vec<Piece>, GameOptions, u16)),
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
