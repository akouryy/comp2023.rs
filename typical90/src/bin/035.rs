#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::{izip, Itertools};
use petgraph::{
    prelude::*,
    stable_graph::IndexType,
    visit::{depth_first_search, DfsEvent, IntoNodeReferences, Time},
};
use proconio::{input, marker::Usize1};
use rand::Rng;
use std::cmp::{max, min};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;
const MOD: usize = 1e9 as usize + 7;

macro_rules! d {
    ($($xs:expr),* $(,)?) => { #[cfg(debug_assertions)]
        eprintln!(concat!("{}: ", $(stringify!($xs), "={:?},  "), *), line!(), $(&$xs),*);
    };
}

/// `ilog2_floor(0)` is `0`.
fn ilog2_floor(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        std::mem::size_of::<usize>() * 8 - 1 - n.leading_zeros() as usize
    }
}
struct LCA {
    parents: Vec<Vec<NodeIndex<usize>>>,
    depths: Vec<usize>,
}
impl LCA {
    fn new(G: &UnGraph<(), (), usize>, root: NodeIndex<usize>) -> Self {
        let mut parents = vec![vec![]; G.node_count()];
        let mut depths = vec![0; G.node_count()];
        depth_first_search(&G, Some(root), |ev| {
            if let DfsEvent::TreeEdge(pi, i) = ev {
                parents[i.index()].push(pi);
                depths[i.index()] = depths[pi.index()] + 1;
            }
        });
        for b in 0..ilog2_floor(G.node_count()) {
            for i in 0..G.node_count() {
                if let Some(j) = parents[i.index()].get(b) {
                    if let Some(&k) = parents[j.index()].get(b) {
                        parents[i].push(k);
                    }
                }
            }
        }
        Self { parents, depths }
    }
    fn lca(&self, i: NodeIndex<usize>, j: NodeIndex<usize>) -> NodeIndex<usize> {
        let (mut i, mut j) = (i, j);
        if self.depths[i.index()] > self.depths[j.index()] {
            std::mem::swap(&mut i, &mut j);
        }
        let d = self.depths[j.index()] - self.depths[i.index()];
        for b in 0..=ilog2_floor(d) {
            if d >> b & 1 == 1 {
                j = self.parents[j.index()][b];
            }
        }
        if i == j {
            return i;
        }
        for b in (0..=ilog2_floor(self.parents.len())).rev() {
            if let Some(&pi) = self.parents[i.index()].get(b) {
                if let Some(&pj) = self.parents[j.index()].get(b) {
                    if pi != pj {
                        i = pi;
                        j = pj;
                    }
                }
            }
        }
        self.parents[i.index()][0]
    }
}

#[deny(dead_code)]
#[proconio::fastout]
fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
        Q: usize,
        mut V: [[Usize1]; Q]
    }

    let G = UnGraph::<(), (), usize>::from_edges(AB);

    let mut preorder_times = vec![Time(UINF); N];
    depth_first_search(&G, Some(0.into()), |ev| {
        if let DfsEvent::Discover(i, t) = ev {
            preorder_times[i.index()] = t;
        }
    });
    let preorder_times = preorder_times;

    let lcaer = LCA::new(&G, 0.into());
    for V in V.iter_mut() {
        V.sort_by_key(|&i| preorder_times[i]);
        let V: &_ = V;

        let lca = V.iter().map(|&V| V.into()).fold1(|i, j| lcaer.lca(i, j)).unwrap();

        let ans = izip!(V, [lca.index()].iter().chain(V.iter()))
            .map(|(&i, &h)| {
                let i = i.into();
                let h = h.into();
                let lca = lcaer.lca(i, h);
                lcaer.depths[i.index()] - lcaer.depths[lca.index()]
            })
            .sum::<usize>();
        println!("{}", ans);
    }
}
