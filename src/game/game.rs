use crate::Tube;
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default)]
pub struct Game {
    time: usize,
    rank: usize,
    empty: usize,
    colors: usize,
    records: Vec<(usize, usize)>,
}

impl Game {
    pub fn new<T: Tube>(c: usize, e: usize) -> (Self, Vec<T>) {
        let new = Self { time: 0, rank: T::size(), empty: e, colors: c, records: vec![] };
        let empties = (0..e).map(|_| T::default()).collect_vec();
        println!("new: {:#?}", new);
        println!("empties: {:#?}", empties);
        unimplemented!()
    }
    pub fn update<T: Tube>(&mut self, _tubes: &mut T) {
        unimplemented!()
    }
    pub fn measure<T: Tube>(&self, _tubes: &[T]) {}
    pub fn win<T: Tube>(&self, tubes: &[T]) -> bool {
        for tubes in tubes {
            if !tubes.empty() && (!tubes.full() || !tubes.sorted()) {
                return false;
            }
        }
        return true;
    }
    pub fn lose<T: Tube>(&self, _tubes: &[T]) -> bool {
        unimplemented!()
    }
    pub fn dump<T: Tube>(&self, tubes: &[T]) -> String {
        let mut out = String::from("#? sort-puzzle\n");

        out.push_str("[puzzle]\n");
        for tube in tubes {
            out.push_str(&format!("{:?}", tube));
        }
        return out;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
