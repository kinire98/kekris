use std::net::IpAddr;

use crate::models::dummy_room::DummyPlayer;

use super::Board;

#[derive(Debug, Clone)]
pub struct RemoteBoard {
    player: DummyPlayer,
}
impl Board for RemoteBoard {
    fn game_over(&self) -> bool {
        todo!()
    }

    fn game_won(&self, _win_condition: impl Fn(bool, u32) -> bool) -> bool {
        todo!()
    }

    fn board_state(&self) -> String {
        todo!()
    }

    fn strategy(&self) -> crate::game::strategy::Strategy {
        todo!()
    }

    fn danger_level(&self) -> super::danger_level::DangerLevel {
        todo!()
    }
}
