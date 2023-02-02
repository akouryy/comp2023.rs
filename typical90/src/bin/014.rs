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
        mut A: [isize; N],
        mut B: [isize; N],
    }

    A.sort();
    B.sort();
    println!("{}", A.iter().zip(B.iter()).map(|(a, b)| (a - b).abs()).sum::<isize>());
}
