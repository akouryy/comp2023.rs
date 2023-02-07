#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::{iproduct, Itertools};
use maplit::hashset;
use num::Num;
use petgraph::prelude::*;
use proconio::{fastout, input, marker::Usize1};
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
#[fastout]
fn main() {
    input! {
        N: usize,
        W: [usize; N],
        B: [usize; N],
    }

    let mut grundy = vec![vec![UINF; 1401]; 51];
    grundy[0][0] = 0;
    grundy[0][1] = 0;

    for (w, b) in iproduct!(0..=50, 0..=1400) {
        let mut used = vec![false; 1401];
        if w > 0 {
            let b2 = b + w;
            if b2 <= 1400 {
                used[grundy[w - 1][b2]] = true;
            }
        }
        if b >= 2 {
            for k in 1..=(b / 2) {
                used[grundy[w][b - k]] = true;
            }
        }
        grundy[w][b] = (0..=1400).find(|&i| !used[i]).unwrap();
    }

    let acc = W.iter().zip(B.iter()).map(|(&w, &b)| grundy[w][b]).fold(0, |acc, x| acc ^ x);
    println!("{}", if acc > 0 { "First" } else { "Second" })
}
