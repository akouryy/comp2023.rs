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
use num_integer::Roots;
use petgraph::prelude::*;
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

fn eratosthenes(n: usize) -> Vec<usize> {
    if n <= 1 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n.sqrt() {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| i).collect()
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    let mut cnt = vec![0; N + 1];

    for &p in eratosthenes(N).iter() {
        for i in (p..=N).step_by(p) {
            cnt[i] += 1;
        }
    }
    println!("{}", cnt.iter().filter(|&&c| c >= K).count());
}
