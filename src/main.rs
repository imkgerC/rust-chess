extern crate core;

fn main() {
    let g = core::game_representation::Game::from_pgn(
        r#"[Event "?"]
           [Site "?"]
           [Date "????.??.??"]
           [Round "?"]
           [White "?"]
           [Black "?"]
           [Result "*"]
           
           1. e4 c5 2. Nf3 d6 3. d4 cxd4 4. Nxd4 Nf6 5. Nc3 a6 6. Be2 e5 7. Nb3 Be7 8. O-O O-O *"#,
    )
    .unwrap();
    println!("{}", g.to_fen());
}
