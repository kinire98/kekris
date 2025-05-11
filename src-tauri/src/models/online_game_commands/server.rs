use serde::{Deserialize, Serialize};

use crate::{game::pieces::Piece, models::dummy_room::DummyPlayer};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerOnlineGameCommands {
    TrashSent(u32),
    Queue(Vec<Piece>),
    BoardStateRequest,
    DangerLevelRequest,
    Won,
    PlayerLost(DummyPlayer),
}
