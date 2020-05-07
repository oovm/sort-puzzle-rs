use crate::{Game, Tube};
use std::collections::BTreeMap;
use rand::seq::SliceRandom;

impl Game {
    pub fn try_swap<T: Tube>(&mut self, tubes: &[T], swap: (usize, usize)) -> usize {
        let mut tubes = tubes.to_vec().clone(); // todo: avoid alloc
        let mut tl = tubes[swap.0].clone(); //todo: better swap
        let mut tr = tubes[swap.1].clone();
        tl.move_to(&mut tr);
        tubes[swap.0] = tl;
        tubes[swap.1] = tr;
        let new = self.measure(&tubes);
        return new;
    }
    // 这其实是个带子结构的搜索
    // 比如 (1 1 2 2) (2, 2, 1, 1) 和 (3, 3, 1, 1) (1 1 3 3) 就是同形的
    // 而且已经排完的可以直接剔除
    // 有很多减枝可以做
    pub fn random_next<T: Tube>(&mut self, tubes: &mut Vec<T>) -> bool {
        let moves = self.smart_available(tubes);
        let new = match moves.choose(&mut rand::thread_rng()) {
            None => return false,
            Some(s) => s,
        };
        self.records.push(*new);
        let mut tl = tubes[new.0].clone();
        let mut tr = tubes[new.1].clone();
        println!("{} -> {}", new.0, new.1);
        // println!("{:?}", tl);
        // println!("{:?}", tr);
        tl.move_to(&mut tr);
        tubes[new.0] = tl;
        tubes[new.1] = tr;
        return self.win(tubes)
    }

    pub fn greedy_next<T: Tube>(&mut self, tubes: &mut Vec<T>) -> bool {
        let mut moves = self.smart_available(tubes);
        let (mut i, mut j) = match moves.pop() {
            // lose and stop
            None => return false,
            Some(s) => s,
        };
        let mut max = self.try_swap(tubes, (i, j));
        for (from, to) in moves {
            let new = self.try_swap(tubes, (from, to));
            if new >= max {
                i = from;
                j = to;
                max = new
            }
        }
        // todo: better swap
        self.records.push((i, j));
        let mut tl = tubes[i].clone();
        let mut tr = tubes[j].clone();
        // println!("{} -> {}", i, j);
        // println!("{:?}", tl);
        // println!("{:?}", tr);
        tl.move_to(&mut tr);
        tubes[i] = tl;
        tubes[j] = tr;
        return self.win(tubes)
    }
    #[rustfmt::skip]
    pub fn smart_available<T: Tube>(&self, tubes: &[T]) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(tubes.len().pow(2));
        for (i, l) in tubes.iter().enumerate() {
            for (j, r) in tubes.iter().enumerate() {
                if i != j // not self
                    && l.can_move(r) // valid move, impl  !l.empty() + !r.full()
                    && !(l.homogenous() && r.empty()) // don't move from homogenous
                    && !(l.homogenous() && r.homogenous() && l.count() > r.count()) // don't move from ordered
                    && !(l.sorted()) // don't move from sorted
                {
                    out.push((i, j))
                }
            }
        }
        return out;
    }
    pub fn solve<T: Tube>(&mut self, tubes: &mut Vec<T>) {
        if self.win(tubes) {
            return;
        }
        for _ in 0..10000 {
            if self.random_next(tubes) {
                return;
            }
            if self.win(tubes) {
                return;
            }
        }
        println!("fail to solve")
    }
}
