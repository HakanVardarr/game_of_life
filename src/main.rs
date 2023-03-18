use game_of_life::game_init;
use game_of_life::Board;
use game_of_life::GameOfLife;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 4 {
        eprintln!("ERROR: Not enough arguments");
        std::process::exit(0);
    }

    let height = args[1].parse::<usize>().unwrap();
    let width = args[2].parse::<usize>().unwrap();

    let mut game = GameOfLife::new(Board::new(height, width));
    game_init(&mut game, &args[3]);
}
