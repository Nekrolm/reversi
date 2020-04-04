use crate::player::{Player, MoveResponse};
use crate::game::board::{PlayerId, Board, MoveError};

use std::rc::Rc;
use crate::view::BoardView;

pub struct UserPlayer {
    id : PlayerId,
    view : Rc<dyn BoardView>
}

impl UserPlayer {
    pub fn new(id : PlayerId, view : Rc<dyn BoardView>) -> Self {
        return Self {id, view}
    }
}

impl Player for UserPlayer {
    fn player_id(&self) -> PlayerId {
        return self.id;
    }
    fn request_move(&mut self, board : &Board) -> MoveResponse {
        return self.view.input(self.player_id(), board)
    }
    fn notify_error(&mut self, err : MoveError) {
        self.view.handle_error(err);
    }
    fn send_result(&mut self, my_score : u32, other_score : u32) {
        self.view.handle_result(my_score, other_score);
    }
}