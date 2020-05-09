use sort_puzzle::{State};



fn main(){
    let new = State::new(12,2);
    let result = new.dfs_solve();
    println!("{:#?}", result.unwrap().len())
}
