#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::Itertools;
use num::Num;
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
        _N: usize,
        A: [isize; _N * 2],
    }
    let M = _N * 2;

    let mut dp = vec![vec![IINF; M + 1]; M + 1];
    for i in 0..=M {
        dp[i][i] = 0;
    }
    for d in 0..=M {
        if d % 2 == 1 {
            continue;
        }
        for i in 0..=M - d {
            let j = i + d;
            if j >= 2 {
                if i < M {
                    dp[i][j] = min(dp[i][j], dp[i + 1][j - 1] + (A[i] - A[j - 1]).abs());
                }
                for k in i + 2..=j - 2 {
                    d!(d, i, j, k);
                    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
                }
            }
        }
    }

    println!("{}", dp[0][M]);
}
