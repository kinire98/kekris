use super::Queue;

pub struct RemoteQueue {}

impl Queue for RemoteQueue {
    fn get_piece(&mut self, position: usize) -> Option<crate::game::pieces::Piece> {
        todo!()
    }
}
impl RemoteQueue {
    fn new() -> RemoteQueue {
        todo!()
    }
}
