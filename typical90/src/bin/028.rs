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
use petgraph::prelude::*;
use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
use std::cmp::{max, min};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;
const MOD: usize = 1e9 as usize + 7;

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        COORD: [(usize, usize, usize, usize); N]
    }
    let mut acc = vec![vec![0; 1001]; 1001];
    for &(lx, ly, rx, ry) in COORD.iter() {
        acc[lx][ly] += 1;
        acc[lx][ry] -= 1;
        acc[rx][ly] -= 1;
        acc[rx][ry] += 1;
    }
    for i in 0..1001 {
        for j in 0..1000 {
            acc[i][j + 1] += acc[i][j];
        }
    }
    for j in 0..1001 {
        for i in 0..1000 {
            acc[i + 1][j] += acc[i][j];
        }
    }
    let mut ans = vec![0; N];
    for line in acc.iter() {
        for a in line.iter() {
            if *a > 0 {
                ans[*a - 1] += 1;
            }
        }
    }
    println!("{}", ans.iter().join("\n"));
}
