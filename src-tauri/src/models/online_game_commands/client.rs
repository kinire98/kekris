use serde::{Deserialize, Serialize};

use crate::game::{board::danger_level::DangerLevel, strategy::Strategy};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ClientOnlineGameCommands {
    TrashSent(Strategy, u32),
    BoardState(String),
    DangerLevel(DangerLevel),
    Lost(u8), // Those number serve no purpose but have content to serialize and deserialize
    QueueRequest(u8), // Those number serve no purpose but have content to serialize and deserialize
}
