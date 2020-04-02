
use std::ops::{Index, IndexMut, Range};
use std::error::Error;


pub const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum PlayerId {
    Black,
    White,
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum MoveError {
    CellNotEmpty,
    MoveHasNoEffect,
    InvalidCell
}

fn flip(p: PlayerId) -> PlayerId {
    return match p {
        PlayerId::Black => PlayerId::White,
        PlayerId::White => PlayerId::Black,
    }
}

pub type CellState = Option<PlayerId>;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Cell {
    x: i32,
    y: i32
}

impl Cell {
    pub fn new(x : i32, y : i32) -> Cell {
        return Cell{x, y};
    }
}

type Direction = Cell;


pub struct Board {
    field: [[CellState; BOARD_SIZE]; BOARD_SIZE]
}

impl Index<Cell> for Board {
    type Output = CellState;
    fn index(&self, cell: Cell) -> &Self::Output {
        return &(self.field[cell.x as usize][cell.y as usize]);
    }
}

impl IndexMut<Cell> for Board {
    fn index_mut(&mut self, cell: Cell) -> &mut Self::Output {
        return &mut (self.field[cell.x as usize][cell.y as usize]);
    }
}


macro_rules! dir {
($x:expr, $y:expr) => {
    Direction{x:$x,y:$y}
};
}

const DIRECTIONS: [Direction; 8] = [dir!(-1,-1), dir!(-1, 0), dir!(-1, 1),
    dir!(0, -1), /* pivot */  dir!(0, 1),
    dir!(1, -1), dir!(1, 0), dir!(1, 1)];

const VALID_INDEX_RANGE: Range<i32> = 0i32..(BOARD_SIZE as i32);

pub fn advance(cell: Cell, dir: Direction) -> Cell {
    return Cell { x: cell.x + dir.x, y: cell.y + dir.y }
}


impl Board {
    pub fn new() -> Board {
        let mut field = [[CellState::None; BOARD_SIZE]; BOARD_SIZE];
        let mid = BOARD_SIZE / 2;
        field[mid-1][mid-1] = Some(PlayerId::Black);
        field[mid][mid] = Some(PlayerId::Black);
        field[mid][mid-1] = Some(PlayerId::White);
        field[mid-1][mid] = Some(PlayerId::White);
        return Board{field};
    }

    pub fn size(&self) -> usize {
        return BOARD_SIZE;
    }

    pub fn is_valid_cell(&self, cell: Cell) -> bool {
        return VALID_INDEX_RANGE.contains(&cell.x)
            && VALID_INDEX_RANGE.contains(&cell.y)
    }

    fn check_direction(&self, start: Cell, direction: Direction, player: PlayerId) -> bool {
        let mut cell = advance(start, direction);
        let mut cell_counter = 0;
        while self.is_valid_cell(cell) && match self[cell] {
            Some(filled) => player == filled,
            _ => false
        } {
            cell_counter += 1;
            cell = advance(cell, direction);
        }
        return self.is_valid_cell(cell) && cell_counter > 0 && !self[cell].is_none()
    }

    fn check_move(&self, cell: Cell, player: PlayerId) -> bool {
        let other_player = flip(player);
        return DIRECTIONS.iter().any(
            |dir| self.check_direction(cell, *dir, other_player));
    }


    pub fn can_move_cell(&self, cell: Cell, player: PlayerId) -> Option<MoveError> {
        use MoveError::{InvalidCell, CellNotEmpty, MoveHasNoEffect};

        if !self.is_valid_cell(cell) {
            return Some(InvalidCell);
        }
        match self[cell] {
            Some(_) => Some(CellNotEmpty),
            None => if self.check_move(cell, player) {
                None
            } else {
                Some(MoveHasNoEffect)
            }
        }
    }

    pub fn available_moves(&self, player: PlayerId) -> Vec<Cell> {
        let mut moves = Vec::new();
        for x in VALID_INDEX_RANGE {
            for y in VALID_INDEX_RANGE {
                let cell = Cell { x, y };
                if self.can_move_cell(cell, player).is_none() {
                    moves.push(cell)
                }
            }
        }
        return moves;
    }

    pub fn count(&self, player: PlayerId) -> u32 {
        let mut cnt = 0u32;
        for x in VALID_INDEX_RANGE {
            for y in VALID_INDEX_RANGE {
                let cell = Cell { x, y };
                match self[cell] {
                    Some(filled) => if filled == player { cnt += 1},
                    _ => continue
                }
            }
        }
        return cnt;
    }

    fn apply_move_direction(&mut self, start: Cell, direction: Direction, player: PlayerId) {
        let mut cell = advance(start, direction);
        if !self.check_direction(start, direction, flip(player)) {
            return;
        }
        while self.is_valid_cell(cell) && match self[cell] {
            Some(filled) => player != filled,
            None => false
        } {
            self[cell] = Some(player);
            cell = advance(cell, direction);
        }
    }

    fn apply_move(&mut self, cell: Cell, player: PlayerId) {
        self[cell] = Some(player);
        for dir in &DIRECTIONS {
            self.apply_move_direction(cell, *dir, player);
        }
    }

    pub fn try_move(&mut self, cell: Cell, player: PlayerId) -> Option<MoveError> {
        return match self.can_move_cell(cell, player) {
            None => {
                self.apply_move(cell, player);
                return None;
            },
            err => err
        }
    }

    pub fn can_move(&self, player: PlayerId) -> bool {
        return !self.available_moves(player).is_empty();
    }
}
