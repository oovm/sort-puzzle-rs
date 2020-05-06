use crate::Tube;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    fmt::{self, Display, Formatter},
    iter::repeat,
};

#[derive(Debug, Default)]
pub struct Game {
    pub time: usize,
    pub rank: usize,
    pub empty: usize,
    pub colors: usize,
    pub records: Vec<(usize, usize)>,
}

impl Game {
    pub fn new<T: Tube>(c: usize, e: usize) -> (Self, Vec<T>) {
        let new = Self { time: 0, rank: T::size(), empty: e, colors: c, records: vec![] };
        let mut store = Vec::with_capacity(T::size() * c);
        let mut out = Vec::with_capacity(c + e);
        for mut i in (1u8..=c as u8).map(|e| repeat(e).take(T::size()).collect_vec()) {
            store.append(&mut i)
        }
        store.shuffle(&mut thread_rng());
        for i in store.chunks(T::size()) {
            out.push(T::new(i));
        }
        out.append(&mut (0..e).map(|_| T::default()).collect_vec());
        (new, out)
    }
    pub fn measure<T: Tube>(&self, tubes: &[T]) -> usize {
        let mut score = 0;
        let mut cnt = 1;
        for tube in tubes {
            if tube.empty() {
                score += 10
            }
            else {
                let mut tube = (*tube).clone();
                let mut c1 = tube.pop();
                while !tube.empty() {
                    let c2 = tube.pop();
                    if c1 == c2 {
                        cnt += 1
                    }
                    else {
                        c1 = c2;
                        cnt = 1
                    }
                }
                score += 5 * cnt
            }
        }
        return score;
    }

    pub fn available<T: Tube>(&self, tubes: &[T]) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(tubes.len().pow(2));
        for (i, l) in tubes.iter().enumerate() {
            for (j, r) in tubes.iter().enumerate() {
                if i != j && l.can_move(r) {
                    out.push((i, j))
                }
            }
        }
        return out;
    }

    pub fn win<T: Tube>(&self, tubes: &[T]) -> bool {
        for tube in tubes {
            if !tube.empty() && !tube.sorted() {
                return false;
            }
        }
        return true;
    }
    pub fn lose<T: Tube>(&self, _tubes: &[T]) -> bool {
        unimplemented!()
    }
    pub fn dump<T: Tube>(&self, tubes: &[T]) -> String {
        let mut out = format!("{:?}", self);
        let mut _insert = String::from("[puzzle]\n");
        for tube in tubes {
            out.push_str(&format!("{:?}", tube));
        }
        return out;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "game = sort-puzzle\n")?;
        write!(f, "mode = {} * {} + {}", self.colors, self.rank, self.empty)
    }
}
