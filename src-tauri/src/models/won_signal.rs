use serde::{Deserialize, Serialize};

use super::dummy_room::DummyPlayer;

/// `WonSignal` represents a signal that a player has won the game.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WonSignal {
    /// The player who won the game.
    pub player: DummyPlayer,
    /// A boolean indicating whether the player is the host of the game.
    pub is_hosting: bool,
}
