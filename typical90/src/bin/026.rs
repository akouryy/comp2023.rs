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
use petgraph::{algo::dijkstra, prelude::*};
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
        AB: [(Usize1, Usize1); N - 1],
    }

    let G = UnGraph::<(), (), usize>::from_edges(&AB);
    let ds = dijkstra(&G, 0.into(), None, |_| 1);
    let rem = if ds.iter().filter(|(_, &d)| d % 2 == 0).count() >= N / 2 { 0 } else { 1 };

    println!("{}", (0..N).filter(|&i| ds[&i.into()] % 2 == rem).take(N / 2).map(|i| i + 1).join(" "))
}
