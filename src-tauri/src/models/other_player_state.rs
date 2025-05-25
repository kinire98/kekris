use serde::{Deserialize, Serialize};

use super::dummy_room::DummyPlayer;

#[derive(Serialize, Deserialize, Clone)]
pub struct OtherPlayerState {
    pub player: DummyPlayer,
    pub state: String,
}
