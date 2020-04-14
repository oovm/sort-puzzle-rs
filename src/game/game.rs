use crate::Tube;

#[derive(Debug, Default)]
pub struct Game {
    time: usize,
    rank: usize,
    empty: usize,
    colors: usize,
    records: Vec<(usize, usize)>,
}

pub struct State<T:Tube> {
    score: usize,
    tubes: Vec<T>
}

impl<T> State<T> {
    fn win() {
        unimplemented!()
    }
    fn lose(){
        unimplemented!()
    }
}

impl Game {
    pub fn new<T: Tube>(r: usize) -> (Self, State<T>) {
       unimplemented!()
    }
    pub fn update<T: Tube>(&mut self, board: &mut T) {
        unimplemented!()
    }
}
