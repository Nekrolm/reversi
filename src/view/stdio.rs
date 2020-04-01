use crate::game::player::Player;
pub use crate::game::player::PlayerId;

use crate::game::board;
use crate::game::board::Cell;

use std::io;
use std::ops::Range;

pub struct StdIOPlayer {
    id : PlayerId
}

impl StdIOPlayer {
    pub fn new (player : PlayerId) -> StdIOPlayer {
        return StdIOPlayer{id : player};
    }
}


fn draw_cell(cell : board::CellState){
    print!("{}", match cell {
        None => "| ",
        Some(PlayerId::Black) => "|X",
        Some(PlayerId::White) => "|O",
    })
}

fn draw_board(board: &board::Board) {
    for x in 0..board.size() as i32{
        for y in 0..board.size() as i32{
            draw_cell(board[Cell::new(x, y)])
        }
        println!("|")
    }
}

impl Player for StdIOPlayer {
    fn player_id(&self) -> PlayerId {
        return self.id;
    }

    fn request_move(&mut self, board : &board::Board) -> Cell {
        draw_board(board);
        println!("{} move: ", match self.id {
            PlayerId::White => "O",
            PlayerId::Black => "X",
        });

        let mut input = String::new();


        io::stdin().read_line(& mut input).unwrap();
        let strs = input.split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap());
        let v : Vec<i32> = strs.collect();
        return Cell::new( v[0], v[1]);
    }

    fn notify_error(&mut self, err : board::MoveError){
        println!("errr!");
    }
    fn send_result(&mut self, my_score : u32, other_score : u32){
        println!("Result: {} - {}", my_score, other_score);
    }
}