#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const INF: usize = 1 << 60;

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars,
    }

    let mut pq = BinaryHeap::from(
        S.iter().take(N - K + 1).enumerate().map(|(i, &c)| (Reverse(c), Reverse(i))).collect_vec(),
    );

    let mut ans = Vec::with_capacity(100);
    let mut last_index = 0;
    while let Some((Reverse(c), Reverse(i))) = pq.pop() {
        if last_index <= i {
            ans.push(c);
            if ans.len() == K {
                break;
            }
            last_index = i;
            let j = N - K + ans.len();
            pq.push((Reverse(S[j]), Reverse(j)));
        }
    }

    println!("{}", ans.iter().join(""));
}
