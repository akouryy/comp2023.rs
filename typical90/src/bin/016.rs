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
    input! { N: isize, A: isize, B: isize, C: isize }
    let mut ans = IINF;
    for i in 0..=min(9999, N / A) {
        let rest = N - i * A;
        for j in 0..=min(9999, rest / B) {
            let rest = rest - j * B;
            if rest >= 0 && rest % C == 0 {
                ans = min(ans, i + j + rest / C);
            }
        }
    }
    println!("{}", ans);
}
