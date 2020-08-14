extern crate core;

fn main() {
    let mut g = core::game_representation::Game::startpos();
    let a = core::move_generation::Action::from_san("e2e4", &g).unwrap();
    g.execute_action(&a);
    let a = core::move_generation::Action::from_san("c5", &g).unwrap();
    g.execute_action(&a);
    let a = core::move_generation::Action::from_san("Nf3", &g).unwrap();
    g.execute_action(&a);
    println!("{}", g.to_fen());
}
