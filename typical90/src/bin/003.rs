#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};

const INF: usize = 1 << 60;

trait ArgOrd<T> {
    fn amax(&self) -> Option<usize>;
    fn amin(&self) -> Option<usize>;
}

impl<T: Ord> ArgOrd<T> for [T] {
    fn amax(&self) -> Option<usize> {
        (0..self.len()).max_by_key(|&i| &self[i])
    }
    fn amin(&self) -> Option<usize> {
        (0..self.len()).min_by_key(|&i| &self[i])
    }
}

trait TreeExt {
    fn tree_diameter_nlogn(&self) -> usize;
}

impl<N, E, Ix> TreeExt for UnGraph<N, E, Ix>
where
    Ix: IndexType,
{
    fn tree_diameter_nlogn(&self) -> usize {
        let src = self.node_references().next().unwrap().0;
        let src = *dijkstra(&self, src, None, |_| 1).iter().max_by_key(|e| e.1).unwrap().0;
        *dijkstra(&self, src, None, |_| 1).iter().max_by_key(|e| e.1).unwrap().1
    }
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
    }

    let ans = UnGraph::<(), (), usize>::from_edges(AB).tree_diameter_nlogn() + 1;
    println!("{}", ans);
}
