use serde::{Deserialize, Serialize};

use crate::{game::pieces::Piece, models::dummy_room::DummyPlayer};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerOnlineGameCommands {
    TrashSent(u32),
    Queue(Vec<Piece>),
    Won,
    PlayerLost(DummyPlayer),
    GameEnded(DummyPlayer),
    State(DummyPlayer, String),
}
