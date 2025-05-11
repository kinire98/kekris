use serde::{Deserialize, Serialize};

use crate::game::{board::danger_level::DangerLevel, strategy::Strategy};

#[derive(Clone, Deserialize, Serialize)]
pub enum ClientOnlineGameCommands {
    TrashSent(Strategy, u32),
    BoardState(String),
    DangerLevel(DangerLevel),
    Lost,
    QueueRequest,
}
