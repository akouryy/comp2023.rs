#![allow(dead_code, non_snake_case, unused_imports, unused_macros, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand::Rng;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    mem::swap,
};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;
const MOD: isize = 1e9 as isize + 7;

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
        H: u8,
        W: u8,
        C: [Chars; H],
    }

    // キー: 最下位(第0)bit: 下に駒があるか, 第Wbit: そこに駒があるか
    let mut dp = HashMap::<usize, isize>::new();
    dp.insert(0, 1);

    for i in 0..H {
        for j in 0..W {
            let dq = dp;
            dp = HashMap::new();
            for (&bb, &d) in dq.iter() {
                let x = dp.entry(bb >> 1).or_default();
                *x = (*x + d) % MOD;

                if C[i as usize][j as usize] == '.'
                    && (j == 0 || bb & 1 == 0 && bb & 1 << W == 0)
                    && bb & 2 == 0
                    && (j == W - 1 || bb & 4 == 0)
                {
                    let x = dp.entry(bb >> 1 | 1 << W).or_default();
                    *x = (*x + d) % MOD;
                }
            }
            d!(dp);
        }
    }
    println!("{}", dp.values().fold(0, |a, b| (a + b) % MOD));
}
