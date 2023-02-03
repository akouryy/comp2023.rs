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
use std::{
    cmp::{max, min},
    collections::HashSet,
};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;
const MOD: usize = 1e9 as usize + 7;

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }

    let mut used = HashSet::new();

    for (i, s) in S.iter().enumerate() {
        if used.insert(s) {
            println!("{}", i + 1)
        }
    }
}
