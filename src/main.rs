use sort_puzzle::State;


#[test]
fn dfs() {
    let new = State::new(12, 2);
    let result = new.dfs_solve();
    println!("{:#?}", result.unwrap().len());
}

#[test]
fn astar() {
    let new = State::new(10, 2);
    let result = new.astar_solve();
    println!("{:#?}", result.unwrap().0.len());
}

fn main() {
    let new = State::new(12, 2);
    let result = new.dfs_solve();
    println!("{:#?}", result.unwrap().len())
}
