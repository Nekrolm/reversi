pub mod stdio;
pub mod curses;



use crate::player::{MoveResponse, PlayerId};
use crate::game::board::{Board, MoveError};

pub trait BoardView {
    fn input(& self, player : PlayerId, board : &Board) -> MoveResponse;
    fn handle_error(& self, err : MoveError) {}

    fn handle_result(& self, my_score : u32, other_score : u32) {}
}