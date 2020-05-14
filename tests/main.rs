use sort_puzzle::{Game, Tube4};

#[test]
fn test() {
    let (game, state) = Game::new::<Tube4>(10, 2);
    println!("{:#?}", state);
    println!("{:#?}", game.measure(&state));
}

#[test]
fn test2() {
    let (game, state) = Game::new::<Tube4>(5, 2);
    println!("{:#?}", state);
    println!("{:#?}", game.available(&state));
}
