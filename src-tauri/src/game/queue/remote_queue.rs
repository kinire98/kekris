use super::Queue;

pub struct RemoteQueue {}

impl Queue for RemoteQueue {
    fn get_piece(&mut self, _position: usize) -> Option<crate::game::pieces::Piece> {
        todo!()
    }
}
impl RemoteQueue {
    fn _new() -> RemoteQueue {
        todo!()
    }
}
