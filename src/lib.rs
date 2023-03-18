mod board;
mod cell;
mod game;
mod state;

pub use board::Board;
pub use cell::Cell;
pub use game::GameOfLife;
use std::io::Read;

pub fn game_init(game: &mut GameOfLife, file: &String) {
    let mut file = std::fs::File::open(file).expect("ERROR: File not found");
    let mut text = String::new();

    file.read_to_string(&mut text).unwrap();
    for line in text.lines() {
        let values = line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        game.board.cells[values[0]][values[1]] = Cell::live();
    }

    loop {
        game.render();
        game.update();
    }
}
