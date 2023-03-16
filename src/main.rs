use game_of_life::game_init;
use game_of_life::Board;
use game_of_life::GameOfLife;

fn main() {
    let mut game = GameOfLife::new(Board::new(30, 100));
    game_init(&mut game);
}
