#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::{iproduct, Itertools};
use petgraph::{prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{input, marker::Usize1};
use rand::Rng;
use std::{
    cmp::{max, min},
    collections::HashSet,
    iter::FromIterator,
};

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
        A: [[isize; N]; N],
        M: usize,
        XY: [(Usize1, Usize1); M],
    }
    let XY: HashSet<(usize, usize)> = XY.into_iter().collect();

    // dp[走った人の集合][直前の人]
    let mut dp = vec![vec![IINF; N]; 1 << N];
    dp[0][0] = 0;
    for (b, i) in iproduct!(0..(1 << N), 0..N) {
        for j in 0..N {
            if b & 1 << j == 0 && (b == 0 || i != j && !XY.contains(&(min(i, j), max(i, j)))) {
                dp[b | 1 << j][j] = min(dp[b | 1 << j][j], dp[b][i] + A[j][b.count_ones() as usize]);
            }
        }
    }
    d!(dp);
    println!("{}", dp[(1 << N) - 1].iter().filter(|&&x| x < IINF).min().unwrap_or(&-1));
}
