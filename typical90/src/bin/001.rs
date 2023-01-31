#![allow(non_snake_case, clippy::uninlined_format_args)]

#[allow(unused_imports)]
use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        N: usize,
        L: usize,
        K: usize,
        mut A: [usize; N],
    }
    A.push(L);

    let mut ok = 0;
    let mut ng = L + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut last = 0;
        let mut cnt = 0;
        for &a in A.iter() {
            if a - last >= mid {
                last = a;
                cnt += 1;
            }
        }
        eprintln!("{}: {}", mid, cnt);
        if cnt >= K + 1 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
