use crate::player::{Player, MoveResponse};
use crate::game::board::{PlayerId, Board, MoveError};

use rand::seq::SliceRandom;

pub struct DummyPlayer {
    id : PlayerId,
    rnd : rand::rngs::ThreadRng,
}

impl DummyPlayer {
    pub fn new(id : PlayerId) -> Self{
        return Self {id, rnd : rand::thread_rng()};
    }
}



impl Player for DummyPlayer {
    fn player_id(&self) -> PlayerId {
        return self.id;
    }

    fn request_move(&mut self, board : &Board) -> MoveResponse {
        return match board.available_moves(self.player_id()).choose(&mut self.rnd) {
            None => MoveResponse::Exit,
            Some(cell) => MoveResponse::Move(*cell)
        }
    }

    fn notify_error(&mut self, err : MoveError) {}
    fn send_result(&mut self, my_score : u32, other_score : u32) {}
}