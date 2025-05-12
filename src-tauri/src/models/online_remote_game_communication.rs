use crate::game::{board::danger_level::DangerLevel, pieces::Piece, strategy::Strategy};

use super::dummy_room::DummyPlayer;

#[derive(Debug, Clone)]
pub enum OnlineToRemoteGameCommunication {
    TrashReceived(DummyPlayer, u32),
    Queue(Vec<Piece>),
    DangerLevelRequest,
    HighestReceivedPlayerRequest,
    Won,
    PlayerLost(DummyPlayer),
    GameEnded(DummyPlayer),
    State(DummyPlayer, String),
}
#[derive(Debug, Clone)]
pub enum RemoteToOnlineGameCommunication {
    TrashSent(DummyPlayer, Strategy, u32),
    BoardState(DummyPlayer, String),
    DangerLevelResponse(DummyPlayer, DangerLevel),
    HighestReceivedPlayer(DummyPlayer, Option<DummyPlayer>),
    Lost(DummyPlayer),
    QueueRequest,
}
