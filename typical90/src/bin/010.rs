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
        CP: [(Usize1, usize); N],
        Q: usize,
        LR: [(Usize1, Usize1); Q],
    }

    let mut acc = vec![vec![0; N + 1]; 2];
    for (i, &(c, p)) in CP.iter().enumerate() {
        acc[c][i + 1] = acc[c][i] + p;
        acc[c ^ 1][i + 1] = acc[c ^ 1][i];
    }

    for (l, r) in LR {
        println!("{}", acc.iter().map(|a| a[r + 1] - a[l]).join(" "));
    }
}
