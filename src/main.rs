use sort_puzzle::State;

#[test]
fn dfs() {
    let new = State::new(4, 2);
    let result = new.dfs_solve();
    println!("{:#?}", result.unwrap().len());
}

fn main() {
    let new = State::new(4, 2);
    let result = new.dfs_solve();
    println!("{:#?}", result.unwrap().len())
}
