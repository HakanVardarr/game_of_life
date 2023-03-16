use std::mem::swap;

use super::board::Board;
use super::cell::Cell;
pub struct GameOfLife {
    pub board: Board,
}

impl GameOfLife {
    pub fn new(board: Board) -> Self {
        Self { board }
    }
    pub fn update(&mut self) {
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                let mut state = Cell::dead();
                let mut count = 0;
                for xo in -1..=1i32 {
                    for yo in -1..=1i32 {
                        let x = x as i32 + xo;
                        let y = y as i32 + yo;

                        if xo == 0 && yo == 0 {
                            continue;
                        }
                        if x < 0
                            || x > self.board.width as i32 - 1
                            || y < 0
                            || y > self.board.height as i32 - 1
                        {
                            continue;
                        }
                        if self.board.cells[y as usize][x as usize].is_alive() {
                            count += 1;
                        }
                    }
                }

                if self.board.cells[y][x].is_alive() && (count == 2 || count == 3) {
                    state = Cell::live();
                }
                if !self.board.cells[y][x].is_alive() && count == 3 {
                    state = Cell::live();
                }
                self.board.new_cells[y][x] = state;
            }
        }

        swap(&mut self.board.cells, &mut self.board.new_cells);
    }
    pub fn render(&self) {
        println!("{}", self.board);
        std::thread::sleep(std::time::Duration::from_millis(320));
        print!("{}[2J", 27 as char);
    }
}
