use serde::{Deserialize, Serialize};

use super::dummy_room::DummyPlayer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WonSignal {
    pub player: DummyPlayer,
    pub is_hosting: bool,
}
