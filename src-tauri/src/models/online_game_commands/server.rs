use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerOnlineGameCommands {
    TrashSent(u32),
}
