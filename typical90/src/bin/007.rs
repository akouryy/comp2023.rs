#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
use std::cmp::{max, min};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;

trait PartitionPoint {
    type Item;
    fn partition_point<F>(&self, f: F) -> usize
    where
        F: Fn(&Self::Item) -> bool;
}
impl<T> PartitionPoint for [T] {
    type Item = T;
    fn partition_point<F>(&self, f: F) -> usize
    where
        F: Fn(&Self::Item) -> bool,
    {
        let mut left_is_ok = 0;
        let mut right_is_ng = self.len();
        while left_is_ok < right_is_ng {
            let mid = (left_is_ok + right_is_ng) / 2;
            if f(&self[mid]) {
                left_is_ok = mid + 1;
            } else {
                right_is_ng = mid;
            }
        }
        left_is_ok
    }
}

#[deny(dead_code)]
#[fastout]
fn main() {
    let mut rng = rand::thread_rng();

    input! {
        N: usize,
        mut A: [isize; N],
        Q: usize,
        B: [isize; Q],
    }

    A.sort();
    let A = A;
    eprintln!("{}", A.iter().max().unwrap());

    if rng.gen() {
        let BI = B.iter().enumerate().sorted_by_key(|(_, &b)| b);
        let mut i = 0;
        let mut ans = vec![0; Q];
        for (j, &b) in BI {
            // `>=` ではなく `>` にしてしまうと、Aに重複要素があったときにそこから進まなくなりWA
            while i + 1 < N && (b - A[i]).abs() >= (b - A[i + 1]).abs() {
                i += 1;
            }
            ans[j] = (b - A[i]).abs();
        }
        println!("{}", ans.iter().join("\n"));
    } else {
        for &b in B.iter() {
            let i = A.partition_point(|&a| a <= b);
            let ans = min(
                (b - A.get(i - 1).unwrap_or(&IINF)).abs(),
                (b - A.get(i).unwrap_or(&IINF)).abs(),
            );
            println!("{}", ans);
        }
    }
}
