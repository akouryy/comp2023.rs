#![allow(non_snake_case, clippy::uninlined_format_args)]

#[allow(unused_imports)]
use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    'b: for b in 0..1 << N {
        let mut lv = 0;
        for i in (0..N).rev() {
            if b & (1 << i) == 0 {
                lv += 1;
            } else {
                lv -= 1;
            }
            if lv < 0 {
                continue 'b;
            }
        }
        if lv == 0 {
            println!("{}", (0..N).rev().map(|i| if b & (1 << i) == 0 { "(" } else { ")" }).join(""))
        }
    }
}
