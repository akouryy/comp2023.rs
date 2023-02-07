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
use std::{
    cmp::{max, min},
    collections::HashMap,
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
        K: usize,
        A: [usize; N],
    }

    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let mut used = HashMap::new();
    while right <= N {
        if used.len() <= K {
            ans = max(ans, right - left);
            if right < N {
                used.entry(A[right]).and_modify(|e| *e += 1).or_insert(1);
            }
            right += 1;
        } else {
            used.entry(A[left]).and_modify(|e| *e -= 1);
            if used[&A[left]] == 0 {
                used.remove(&A[left]);
            }
            left += 1;
        }
    }
    println!("{}", ans);
}
