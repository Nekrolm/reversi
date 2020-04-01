
use crate::game::board;
use crate::game::board::Cell;

pub use board::PlayerId;

pub trait Player {
    fn player_id(&self) -> PlayerId;
    fn request_move(&mut self, board : &board::Board) -> Cell;
    fn notify_error(&mut self, err : board::MoveError);
    fn send_result(&mut self, my_score : u32, other_score : u32);
}