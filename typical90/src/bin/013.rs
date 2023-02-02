#![allow(dead_code, non_snake_case, unused_imports, unused_macros, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
use std::cmp::{max, min};

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

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, usize); M],
    }

    let g = UnGraph::<(), usize, usize>::from_edges(ABC);
    let d1 = dijkstra(&g, 0.into(), None, |e| *e.weight());
    let d2 = dijkstra(&g, (N - 1).into(), None, |e| *e.weight());

    println!("{}", (0..N).map(|i| d1[&i.into()] + d2[&i.into()]).join("\n"));
}
