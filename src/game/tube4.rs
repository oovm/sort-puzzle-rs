use crate::{Tube, Tube4};
use itertools::Itertools;
use pathfinding::{
    directed::{astar::astar, dfs::dfs, idastar::idastar},
    prelude::bfs,
};
use rand::{seq::SliceRandom, thread_rng};
use std::{iter::repeat};

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
        return Self(out);
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
                    && !(l.homogenous() && r.homogenous() && l.count() >= r.count()) // don't move from ordered
                    && !(l.sorted()) // don't move from sorted
                {
                    out.push((i, j))
                }
            }
        }
        return out;
    }

    pub fn swap(&self, p: &(usize, usize)) -> Self {
        let mut tubes = self.0.clone();
        let s = tubes[p.0].pop();
        tubes[p.1].push(s);
        return Self(tubes);
    }

    pub fn successors(&self) -> Vec<State> {
        self.smart_available().iter().map(|p| self.swap(p)).collect_vec()
    }

    pub fn successor_costs(&self) -> Vec<(State, usize)> {
        self.smart_available().iter().map(|p| self.swap(p)).map(|p| (p, 1)).collect_vec()
    }



    pub fn bfs_solve(&self) -> Option<Vec<State>> {
        bfs(self, |p| p.successors(), |p| p.win())
    }

    pub fn dfs_solve(&self) -> Option<Vec<State>> {
        dfs((*self).clone(), |p| p.successors(), |p| p.win())
    }

    pub fn heuristic(&self) -> usize {
        let mut sorted = 0;
        for tube in &self.0 {
            if tube.empty() || tube.sorted() {
                sorted += 0
            }
            else {
                // 外加逆序数?
                sorted += tube.kinds();
            }
        }
        return sorted;
    }

    pub fn astar_solve(&self) -> Option<(Vec<State>, usize)> {
        astar(self, |p| p.successor_costs(), |p| p.heuristic(), |p| p.win())
    }

    pub fn idastar_solve(&self) -> Option<(Vec<State>, usize)> {
        idastar(self, |p| p.successor_costs(), |p| p.heuristic(), |p| p.win())
    }
}
