#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::Itertools;
use petgraph::{prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{input, marker::Usize1};
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
#[proconio::fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        XY: [(isize, isize); N],
        QQ: [Usize1; Q],
    }

    let (slash_min, slash_max) = XY.iter().map(|&(x, y)| x - y).minmax().into_option().unwrap();
    let (backslash_min, backslash_max) = XY.iter().map(|&(x, y)| x + y).minmax().into_option().unwrap();

    for QQ in QQ {
        let (x, y) = XY[QQ];
        let ans = *[x - y - slash_min, slash_max - (x - y), x + y - backslash_min, backslash_max - (x + y)]
            .iter()
            .max()
            .unwrap();
        println!("{}", ans);
    }
}
