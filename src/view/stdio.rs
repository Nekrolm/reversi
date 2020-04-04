use crate::player::MoveResponse;
pub use crate::player::PlayerId;

use crate::game::board;
use crate::game::board::Cell;
use crate::view::BoardView;

use std::io;
use std::ops::Range;

pub struct StdIOView {
}


fn draw_cell(cell : board::CellState){
    print!("{}", match cell {
        None => "| ",
        Some(PlayerId::Black) => "|X",
        Some(PlayerId::White) => "|O",
    })
}

fn draw_board(board: &board::Board) {
    let range = 0..board.size() as i32;
    for x in range.clone() {
        for y in range.clone() {
            draw_cell(board[Cell::new(x, y)])
        }
        println!("|")
    }
}


impl BoardView for StdIOView {
    fn input(&self, player : PlayerId,  board : &board::Board) -> MoveResponse {
        use MoveResponse::{*};
        draw_board(board);
        println!("{} move: ", match player {
            PlayerId::White => "O",
            PlayerId::Black => "X",
        });

        let mut input = String::new();


        io::stdin().read_line(& mut input).unwrap();

        if input.trim() == "exit" {
            return Exit;
        }

        let strs = input.split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap());
        let v : Vec<i32> = strs.collect();
        if v.len() < 2 {
            return Move(Cell::new(-1,-1));
        }
        return Move(Cell::new( v[0], v[1]));
    }

    fn handle_error(&self, err : board::MoveError){
        println!("errr!");
    }
    fn handle_result(&self, my_score : u32, other_score : u32){
        println!("Result: {} - {}", my_score, other_score);
    }
}