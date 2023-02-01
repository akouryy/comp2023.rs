#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};

const INF: usize = 1 << 60;

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }

    let rows = A.iter().map(|r| r.iter().sum::<usize>()).collect_vec();
    let cols = (0..W).map(|j| A.iter().map(|r| r[j]).sum::<usize>()).collect_vec();

    let B = A
        .iter()
        .enumerate()
        .map(|(i, r)| r.iter().enumerate().map(|(j, &a)| rows[i] + cols[j] - a).collect_vec())
        .collect_vec();

    println!("{}", B.iter().map(|r| r.iter().join(" ")).join("\n"))
}
