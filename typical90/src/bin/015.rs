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

fn powmod(a: usize, b: usize, m: usize) -> usize {
    if b == 0 {
        1
    } else if b % 2 == 0 {
        powmod(a * a % m, b / 2, m)
    } else {
        a * powmod(a, b - 1, m) % m
    }
}

fn nCk(fact: &[usize], inv: &[usize], n: usize, k: usize, m: usize) -> usize {
    if
    /*k < 0 ||*/
    k > n {
        0
    } else {
        fact[n] * inv[k] % m * inv[n - k] % m
    }
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut fact = vec![1; N + 1];
    for i in 0..N {
        fact[i + 1] = fact[i] * (i + 1) % MOD;
    }
    let mut inv = vec![1; N + 1];
    inv[N] = powmod(fact[N], MOD - 2, MOD);
    for i in (0..N).rev() {
        inv[i] = inv[i + 1] * (i + 1) % MOD;
    }

    for k in 1..=N {
        let mut ans = 0;
        // N - (i - 1) * (k - 1) == i
        // N == i + (i - 1) * (k - 1) == (i - 1) * k + 1
        for i in 1..=(N - 1) / k + 1 {
            ans += nCk(&fact, &inv, N - (i - 1) * (k - 1), i, MOD);
            ans %= MOD;
        }
        println!("{}", ans);
    }
}
