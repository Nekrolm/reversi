
use std::borrow::{BorrowMut, Borrow};
use std::mem::swap;
use std::ops::Deref;
use futures;
use crate::game::board::Board;

pub mod board;
pub mod player;

struct Players {
    current : Box<dyn player::Player>,
    next :  Box<dyn player::Player>,
}

impl Players {
    fn flip(&mut self){
        swap(& mut self.current, & mut self.next);
    }
}


pub struct  Game  {
    board : board::Board,
    players : Players,
}

fn can_move(player : &dyn player::Player, board : &board::Board) -> bool {
    return board.can_move(player.player_id());
}

fn try_move_cell(player : &dyn player::Player, cell : board::Cell, board : &mut board::Board)
    -> Option<board::MoveError> {
    return board.try_move(cell,player.player_id());
}


impl Game  {
    fn step(&mut self) -> bool {
        match &mut self.players {
            Players {current, next} => {
                if !can_move(current.as_ref(), &self.board){
                    if can_move(next.as_ref(), &self.board){
                        self.players.flip();
                        return true;
                    }
                    return false;
                }
                use player::MoveResponse::{*};
                match current.request_move(&self.board) {
                    Move(cell) => {
                        match try_move_cell(current.as_ref(), cell, &mut self.board) {
                            None => self.players.flip(),
                            Some(err) => current.notify_error(err)
                        }
                        return true;
                    }
                    Exit => false
                }
            }
        }
    }

    pub fn run(& mut self) {
        while self.step() {}
        let current_p_score = self.board.count(self.players.current.player_id());
        let next_p_score = self.board.count(self.players.next.player_id());

        self.players.current.send_result(current_p_score, next_p_score);
        self.players.next.send_result(next_p_score, current_p_score);
    }

    pub fn new(first : Box<dyn player::Player>, second : Box<dyn player::Player>) -> Game {
        return Game{board :Board::new(),
                    players: Players{
                         current:first,
                         next:second}
        }
    }
}
