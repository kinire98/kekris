use super::Board;

#[derive(Debug)]
pub struct RemoteBoard {}
impl Board for RemoteBoard {
    fn game_over(&self) -> bool {
        todo!()
    }

    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool {
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
