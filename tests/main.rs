use itertools::Itertools;
use sort_puzzle::{Game, Tube4};
use std::iter::repeat;

#[test]
fn test() {
    let (game, state) = Game::new::<Tube4>(5, 2);
    println!("{:#?}", state);
    println!("{:#?}", game.measure(&state));
}

#[test]
fn test2() {
    let (game, state) = Game::new::<Tube4>(5, 2);
    println!("{:#?}", state);
    println!("{:#?}", game.available(&state));
}


#[test]
fn test3() {
    let (mut game,  mut state) = Game::new::<Tube4>(5, 2);
    game.next_step(&mut state);
    println!("{:#?}", state);
    println!("{:#?}", game.next_step(&mut state));
}
