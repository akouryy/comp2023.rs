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
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
use std::{
    cmp::{max, min},
    f64::consts::PI,
};

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
#[fastout]
fn main() {
    input! {
        T: f64, L: f64, X: f64, Y: f64, Q: usize,
        E: [f64; Q],
    }

    for &e in E.iter() {
        let theta = e / T * PI * 2.;
        let y = L / 2. * -theta.sin();
        let z = L / 2. - L / 2. * theta.cos();
        println!("{}", z.atan2(X.hypot(Y - y)) / PI * 180.);
    }
}
