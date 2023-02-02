#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rand::Rng;
use std::cmp::{max, min};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;

const MOD: usize = 1e9 as usize + 7;

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: Bytes,
    }

    let atcoder = "atcoder".as_bytes();
    let mut dp = vec![0; 8];
    dp[0] = 1;

    for &c in S.iter() {
        for i in (0..7).rev() {
            if c == atcoder[i] {
                dp[i + 1] += dp[i];
                dp[i + 1] %= MOD;
            }
        }
    }

    println!("{}", dp.last().unwrap());
}
