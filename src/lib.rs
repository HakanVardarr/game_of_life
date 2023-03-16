mod board;
mod cell;
mod game;
mod state;

pub use board::Board;
pub use cell::Cell;
pub use game::GameOfLife;

pub fn game_init(game: &mut GameOfLife) {
    game.board.cells[15][15] = Cell::live();
    game.board.cells[15][16] = Cell::live();
    game.board.cells[16][15] = Cell::live();
    game.board.cells[16][16] = Cell::live();

    game.board.cells[19][15] = Cell::live();
    game.board.cells[19][16] = Cell::live();
    game.board.cells[20][15] = Cell::live();
    game.board.cells[20][16] = Cell::live();

    game.board.cells[13][18] = Cell::live();
    game.board.cells[14][18] = Cell::live();
    game.board.cells[15][18] = Cell::live();
    game.board.cells[16][18] = Cell::live();
    game.board.cells[17][18] = Cell::live();
    game.board.cells[18][18] = Cell::live();
    game.board.cells[19][18] = Cell::live();
    game.board.cells[20][18] = Cell::live();
    game.board.cells[21][18] = Cell::live();
    game.board.cells[22][18] = Cell::live();

    game.board.cells[13][29] = Cell::live();
    game.board.cells[14][29] = Cell::live();
    game.board.cells[15][29] = Cell::live();
    game.board.cells[16][29] = Cell::live();
    game.board.cells[17][29] = Cell::live();
    game.board.cells[18][29] = Cell::live();
    game.board.cells[19][29] = Cell::live();
    game.board.cells[20][29] = Cell::live();
    game.board.cells[21][29] = Cell::live();
    game.board.cells[22][29] = Cell::live();

    game.board.cells[19][31] = Cell::live();
    game.board.cells[19][32] = Cell::live();
    game.board.cells[20][31] = Cell::live();
    game.board.cells[20][32] = Cell::live();

    game.board.cells[12][19] = Cell::live();
    game.board.cells[12][20] = Cell::live();
    game.board.cells[13][20] = Cell::live();
    game.board.cells[23][19] = Cell::live();
    game.board.cells[23][20] = Cell::live();
    game.board.cells[22][20] = Cell::live();

    game.board.cells[12][28] = Cell::live();
    game.board.cells[12][27] = Cell::live();
    game.board.cells[13][27] = Cell::live();

    game.board.cells[23][28] = Cell::live();
    game.board.cells[23][27] = Cell::live();
    game.board.cells[22][27] = Cell::live();

    game.board.cells[16][20] = Cell::live();
    game.board.cells[17][20] = Cell::live();
    game.board.cells[18][20] = Cell::live();
    game.board.cells[19][20] = Cell::live();

    game.board.cells[16][27] = Cell::live();
    game.board.cells[17][27] = Cell::live();
    game.board.cells[18][27] = Cell::live();
    game.board.cells[19][27] = Cell::live();

    game.board.cells[16][23] = Cell::live();
    game.board.cells[16][24] = Cell::live();

    game.board.cells[17][22] = Cell::live();
    game.board.cells[18][22] = Cell::live();

    game.board.cells[19][23] = Cell::live();
    game.board.cells[19][24] = Cell::live();

    game.board.cells[17][25] = Cell::live();
    game.board.cells[18][25] = Cell::live();

    game.board.cells[15][31] = Cell::live();
    game.board.cells[15][32] = Cell::live();
    game.board.cells[16][31] = Cell::live();
    game.board.cells[16][32] = Cell::live();

    loop {
        game.render();
        game.update();
    }
}
