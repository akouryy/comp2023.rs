#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::Itertools;
use lazy_static::lazy_static;
use num::Num;
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

lazy_static! {
    static ref TO9: Vec<isize> = (0..=13).map(|k| 9isize.pow(k)).collect();
    static ref TO10: Vec<isize> = (0..=13).map(|k| 10isize.pow(k)).collect();
}

/// `m - f(m) == B && f(m) != 0`なる`m`のうち、`N`や`fix`を満たすものの個数を返す。
/// - `fix`: `m`の確定した部分。
/// - `prod`: `m`の確定した桁の積。
/// - `k`: `m`は`10**k`の位まで確定している。
fn solve(N: isize, B: isize, fix: isize, prod: isize, k: usize) -> usize {
    let ans = if k == 0 {
        if fix <= N && fix - prod == B {
            d!("A", fix);
            1
        } else {
            // d!("B");
            0
        }
    } else if fix >= N || fix + TO10[k] - 1 < B || fix - prod * TO9[k] > B {
        // d!("C", fix >= N, fix + TO10[k] <= B, fix - prod * TO9[k] > B);
        0
    } else {
        (1..=9).map(|i| solve(N, B, fix + i * TO10[k - 1], prod * i, k - 1)).sum()
    };
    // d!(k, fix, prod, ans);
    ans
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: isize,
        B: isize,
    }

    println!(
        "{}",
        (0..=11).map(|k| solve(N, B, 0, 1, k)).sum::<usize>()
            + if B <= N && B.to_string().contains('0') { 1 } else { 0 }
    );
}
