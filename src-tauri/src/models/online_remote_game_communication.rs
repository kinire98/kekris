use crate::game::{board::danger_level::DangerLevel, pieces::Piece, strategy::Strategy};

use super::dummy_room::DummyPlayer;

#[derive(Debug, Clone)]
pub enum OnlineToRemoteGameCommunication {
    TrashReceived(DummyPlayer, u32),
    Queue(Vec<Piece>),
    BoardStateRequest,
    DangerLevelRequest,
    HighestReceivedPlayerRequest,
    Won,
    PlayerLost(DummyPlayer),
}
#[derive(Debug, Clone)]
pub enum RemoteToOnlineGameCommunication {
    TrashSent(DummyPlayer, Strategy, u32),
    BoardStateResponse(DummyPlayer, String),
    DangerLevelResponse(DummyPlayer, DangerLevel),
    HighestReceivedPlayer(DummyPlayer, DummyPlayer),
    Lost(DummyPlayer),
    QueueRequest,
}
