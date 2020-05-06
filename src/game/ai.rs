use crate::{Game, Tube};
use std::collections::BTreeMap;

impl Game {
    pub fn try_swap<T: Tube>(&mut self, tubes: &[T], swap: (usize, usize)) -> usize {
        let old = self.measure(tubes);
        let mut tubes = tubes.to_vec().clone(); // todo: avoid alloc
        let mut tl = tubes[swap.0].clone(); //todo: better swap
        let mut tr = tubes[swap.1].clone();
        println!("{:?}",tl);
        println!("{:?}",tr);
        tl.move_to(&mut tr);
        tubes[swap.0]= tl;
        tubes[swap.1]= tr;
        let new = self.measure(&tubes);
        return new - old;
    }
    // 这其实是个带子结构的搜索
    // 比如 (1 1 2 2) (2, 2, 1, 1) 和 (3, 3, 1, 1) (1 1 3 3) 就是同形的
    // 而且已经排完的可以直接剔除
    // 有很多减枝可以做
    pub fn next_step<T: Tube>(&mut self, tubes: &mut Vec<T>) {
        let mut max = 0;
        let (mut i, mut j) = (0, 0);
        for (from, to) in self.smart_available(tubes) {
            let new = self.try_swap(tubes, (from, to));
            if new >= max {
                i = from;
                j = to;
                max = new
            }
        }
        //todo: better swap
        self.records.push((i, j));
        let mut tl = tubes[i].clone();
        let mut tr = tubes[j].clone();
        tl.move_to(&mut tr);
        tubes[i]= tl;
        tubes[j]= tr;
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
}
