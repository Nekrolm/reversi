use ncurses;
pub use async_trait::async_trait;
use crate::game::board::{PlayerId, Board, Cell, MoveError, advance};
use ncurses::CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE;
use crate::game::player::Player;
use crate::game::player::MoveResponse;
use ncurses::ll::winch;

pub struct NCursesWindow {
    window : ncurses::WINDOW
}

fn player_str(player : PlayerId) -> char {
    return match player {
        PlayerId::White => 'O',
        PlayerId::Black => 'X',
    };
}

enum Action {
    Exit,
    Accept,
    Offset(Cell)
}

impl NCursesWindow {
    pub fn new() -> Self {
        let window = ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(CURSOR_VERY_VISIBLE);
        ncurses::keypad(window, true);
        ncurses::start_color();
        ncurses::wgetch(window);
        Self { window }
    }

    fn draw(&self, player : PlayerId, board : &Board, pivot : Cell){
        ncurses::wclear(self.window);
        let header = "current move: ".to_owned() + player_str(player).to_string().as_str();
        ncurses::wprintw(self.window, header.as_str());
        ncurses::wprintw(self.window, "\n");
        let range = 0..board.size() as i32;
        for x in range.clone(){
            for y in range.clone() {
                let pos = Cell::new(x, y);
                let mode = if pos == pivot {ncurses::A_STANDOUT()} else {ncurses::A_NORMAL()};
                ncurses::waddch(self.window, match board[pos] {
                    None => '.',
                    Some(filled) => player_str(filled)
                } as u32 | mode);
            }
            ncurses::waddch(self.window, '\n' as u32);
        }
        ncurses::wrefresh(self.window);
    }

    fn action(&self) -> Action {
        const SPACE :i32 = ' ' as i32;
        const EXIT :i32 = 'x' as i32;

        return match ncurses::wgetch(self.window) {
            ncurses::KEY_UP => Action::Offset(Cell::new(-1, 0)),
            ncurses::KEY_DOWN => Action::Offset(Cell::new(1, 0)),
            ncurses::KEY_LEFT => Action::Offset(Cell::new(0, -1)),
            ncurses::KEY_RIGHT => Action::Offset(Cell::new(0, 1)),
            SPACE => Action::Accept,
            EXIT => Action::Exit,
            _ => Action::Offset(Cell::new(0,0))
        }
    }
}

impl Drop for NCursesWindow {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

use std::rc::Rc;

pub struct CursesPlayer {
    id : PlayerId,
    window : Rc<NCursesWindow>,
    position : Cell
}

impl CursesPlayer {
    pub fn new (player : PlayerId, window : Rc<NCursesWindow>) -> Self {
        return Self { id : player ,  window, position : Cell::new(0,0)}
    }
}

impl Player for CursesPlayer {
    fn player_id(&self) -> PlayerId {
        return self.id;
    }

    fn request_move(&mut self, board : &Board) -> MoveResponse {
        loop {
            self.window.draw(self.player_id(), board, self.position);
            match self.window.action() {
                Action::Exit => return MoveResponse::Exit,
                Action::Accept => return MoveResponse::Move(self.position),
                Action::Offset(offset) => {
                    let new_pos = advance(self.position, offset);
                    if board.is_valid_cell(new_pos) {
                        self.position = new_pos;
                    }
                }
            }
        }
        return MoveResponse::Exit;
    }

    fn notify_error(&mut self, err : MoveError){
        println!("errr!");
    }
    fn send_result(&mut self, my_score : u32, other_score : u32){
        println!("Result: {} - {}", my_score, other_score);
    }
}