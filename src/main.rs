extern crate core;

fn main() {
    let b = core::game_representation::Board::startpos();
    println!("{}", b.to_fen());
    let b =
        core::game_representation::Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!("{}", b.to_fen());
}
