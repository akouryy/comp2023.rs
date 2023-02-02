#![allow(dead_code, non_snake_case, unused_imports, unused_macros, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
use std::{cmp::{max, min}, mem::swap};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;
const MOD: usize = 1e9 as usize + 7;

macro_rules! d {
    ($x0:expr $(, $xs:expr)* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!(stringify!($x0), "={:?}", $(",  ", stringify!($xs), "={:?}"), *), &$x0, $(&$xs),*);
    };
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self { par: vec![UINF; n], rank: vec![1; n] }
    }
    fn root(&mut self, i: usize) -> usize {
        if self.par[i] == UINF {
            i
        } else {
            self.par[i] = self.root(self.par[i]);
            self.par[i]
        }
    }
    fn is_same(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }
    fn merge(&mut self, i: usize, j: usize) -> bool {
        let mut i = self.root(i);
        let mut j = self.root(j);
        if i == j {
            return false;
        }
        if self.rank[i] < self.rank[j] {
            swap(&mut i, &mut j);
        }
        self.par[j] = i;
        self.rank[i] += self.rank[j];
        true
    }
}

const D4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        Q: usize,
    }

    let mut uf = UnionFind::new(H * W);
    let mut red = vec![vec![false; W]; H];
    for _ in 0..Q {
        input! { T: usize }
        if T == 1 {
            input! { R: Usize1, C: Usize1 }
            red[R][C] = true;
            for &(di, dj) in &D4 {
                let r = (R as isize + di) as usize;
                let c = (C as isize + dj) as usize;
                if red.get(r).and_then(|x| x.get(c)) == Some(&true) {
                    uf.merge(R * W + C, r * W + c);
                }
            }
        } else {
            input! { RA: Usize1, CA: Usize1, RB: Usize1, CB: Usize1 }
            println!(
                "{}",
                if red[RA][CA] && uf.is_same(RA * W + CA, RB * W + CB) { "Yes" } else { "No" }
            );
        }
    }
}
