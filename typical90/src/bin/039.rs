#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::Itertools;
use petgraph::{
    prelude::*,
    stable_graph::IndexType,
    visit::{depth_first_search, DfsEvent, IntoNodeReferences},
};
use proconio::{input, marker::Usize1};
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

#[deny(dead_code)]
#[proconio::fastout]
fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
    }

    let G = UnGraph::<(), (), usize>::from_edges(AB);
    let mut ans = 0;

    let mut size = vec![0; N];
    depth_first_search(&G, Some(0.into()), |ev| {
        if let DfsEvent::Finish(v, _) = ev {
            let sz = G.neighbors(v).map(|w| size[w.index()]).sum::<usize>() + 1;
            size[v.index()] = sz;
            ans += sz * (N - sz);
        }
    });

    println!("{}", ans);
}
