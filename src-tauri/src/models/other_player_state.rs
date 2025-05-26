use serde::{Deserialize, Serialize};

use super::dummy_room::DummyPlayer;

/// `OtherPlayerState` represents the state of another player in the game.
#[derive(Serialize, Deserialize, Clone)]
pub struct OtherPlayerState {
    /// The player's information.
    pub player: DummyPlayer,
    /// The state of the player's board as a string.
    pub state: String,
}
