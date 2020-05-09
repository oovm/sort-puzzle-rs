
use pathfinding::prelude::bfs;
use crate::{Tube4, Tube};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::repeat;
use itertools::Itertools;
use pathfinding::directed::dfs::dfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State(Vec<Tube4>);

impl State {
    pub fn new(c: usize, e: usize) -> Self {
        let mut store = Vec::with_capacity(Tube4::size() * c);
        let mut out = Vec::with_capacity(c + e);
        for mut i in (1u8..=c as u8).map(|e| repeat(e).take(Tube4::size()).collect_vec()) {
            store.append(&mut i)
        }
        store.shuffle(&mut thread_rng());
        for i in store.chunks(Tube4::size()) {
            out.push(Tube4::new(i));
        }
        out.append(&mut (0..e).map(|_| Tube4::default()).collect_vec());
        return Self(out)
    }

    pub fn win(&self) -> bool {
        for tube in &self.0 {
            if !tube.empty() && !tube.sorted() {
                return false;
            }
        }
        return true;
    }
    #[rustfmt::skip]
    pub fn smart_available(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(self.0.len().pow(2));
        for (i, l) in self.0.iter().enumerate() {
            for (j, r) in self.0.iter().enumerate() {
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

    pub fn swap(&self, p: &(usize, usize))-> Self {
        let mut tubes = self.0.clone();
        let mut tl = tubes[p.0].clone();
        let mut tr = tubes[p.1].clone();
        tl.move_to(&mut tr);
        tubes[p.0] = tl;
        tubes[p.1] = tr;
        return  Self(tubes)
    }

    pub fn successors(&self) -> Vec<State> {
        self.smart_available().iter().map(|p| self.swap(p)).collect_vec()
    }

    pub fn bfs_solve(&self) -> Option<Vec<State>> {
        bfs(self, |p| p.successors(), |p| p.win())
    }

    pub fn dfs_solve(&self) -> Option<Vec<State>> {
        dfs((*self).clone(), |p| p.successors(), |p| p.win())
    }

    pub fn heuristic(&self) -> usize {
        let mut score = 0;
        let mut cnt = 1;
        for tube in self.0 {
            if tube.empty() {
                score += 10
            }
            else {
                let mut tube = self.0.clone();
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

}
