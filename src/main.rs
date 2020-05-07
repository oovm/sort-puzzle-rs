use sort_puzzle::{Game, Tube4};

fn main() {
    let (mut game, mut state) = Game::new::<Tube4>(8, 2);
    println!("{:#?}", state);
    game.solve(&mut state);
    println!("{:#?}", state);
    println!("{}", game);
}
