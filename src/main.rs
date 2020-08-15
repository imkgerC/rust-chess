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
    let g = core::game_representation::Game::from_pgn(
        r#"[Event "?"]
    [Site "?"]
    [Date "????.??.??"]
    [Round "?"]
    [White "?"]
    [Black "?"]
    [Result "*"]
    
    1. e4 c5 2. Nf3 d6 3. d4 cxd4 4. Nxd4 Nf6 5. Nc3 g6 6. Be3 Bg7 7. f3 O-O 8. Qd2 Nc6 *"#,
    )
    .unwrap();
    println!("{}", g.to_fen());
}
