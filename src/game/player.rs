pub use async_trait::async_trait;
use crate::game::board;
use crate::game::board::Cell;

pub use board::PlayerId;

pub enum MoveResponse {
    Move(Cell),
    Exit,
}


#[async_trait]
pub trait Player {
    fn player_id(&self) -> PlayerId;
    async fn request_move(&mut self, board : &board::Board) -> MoveResponse;
    async fn notify_error(&mut self, err : board::MoveError);
    async fn send_result(&mut self, my_score : u32, other_score : u32);
}