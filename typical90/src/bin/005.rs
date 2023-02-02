#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};

const INF: usize = 1 << 60;
const MOD: usize = 1e9 as usize + 7;

fn mul(B: usize, a: &[usize], b: &[usize], left_coef: usize) -> Vec<usize> {
    let mut ans = vec![0; B];
    for i in 0..B {
        let ic = i * left_coef;
        for j in 0..B {
            let k = (ic + j) % B;
            ans[k] += a[i] * b[j];
            ans[k] %= MOD;
        }
    }
    ans
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! { N: usize, B: usize, K: usize, C: [usize; K] }

    let mut coef = vec![0; B];
    for &c in C.iter() {
        coef[c % B] += 1;
    }
    let coef = coef;

    let mut ans = vec![0; B];
    ans[0] = 1;
    let mut pow = coef;
    let mut left_coef = 10 % B;
    let mut n = N;
    while n > 0 {
        if n & 1 == 1 {
            ans = mul(B, &ans, &pow, left_coef);
        }
        pow = mul(B, &pow, &pow, left_coef);
        left_coef = left_coef * left_coef % B;
        n >>= 1;
        // eprintln!("{}: {} {:?} {:?}", n, left_coef, pow, ans);
        // eprintln!("{}", n);
    }

    println!("{}", ans[0]);
}
