#![allow(dead_code, non_snake_case, unused_imports, clippy::uninlined_format_args)]

use itertools::Itertools;
use ordered_float::{NotNan, OrderedFloat};
use petgraph::{algo::dijkstra, prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
use std::{
    cmp::{max, min},
    f64::consts::PI,
};
use superslice::Ext;

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;

const MOD: usize = 1e9 as usize + 7;

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(f64, f64); N],
    }

    let mut ans = NotNan::new(0.).unwrap();
    for &(x0, y0) in XY.iter() {
        let mut angles = XY
            .iter()
            .filter_map(|&(x, y)| {
                if x != x0 || y != y0 {
                    Some(
                        NotNan::new(((y - y0).atan2(x - x0) * 180. / PI).rem_euclid(360.)).unwrap(),
                    )
                } else {
                    None
                }
            })
            .collect_vec();
        angles.sort();
        let angles = angles;
        for &a in angles.iter() {
            let i1 = angles.lower_bound(&((a + 180.) % 360.));
            for &i in &[i1 - 1, i1] {
                let b = angles[i.rem_euclid(angles.len())];
                if (a - b).abs() > EPS {
                    // eprintln!("  a={}, b={} => {}", a, b, min((b + 360. - a) % 360., (a + 360. - b) % 360.));
                    ans = max(ans, min((b + 360. - a) % 360., (a + 360. - b) % 360.));
                }
            }
        }
        // eprintln!("  ans={}", ans);
    }
    println!("{}", ans);
}
