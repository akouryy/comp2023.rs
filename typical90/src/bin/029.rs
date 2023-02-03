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
    fmt,
    ops::Range,
};

const IINF: isize = 1 << 60;
const UINF: usize = 1 << 60;
const EPS: f64 = 1e-20;
const MOD: usize = 1e9 as usize + 7;

trait Monoid {
    type Val: Clone + Eq;
    fn op(a: &Self::Val, b: &Self::Val) -> Self::Val;
    fn id() -> Self::Val;
}
struct MaxMonoid {}
impl Monoid for MaxMonoid {
    type Val = usize;
    fn op(a: &usize, b: &usize) -> usize {
        *a.max(b)
    }
    fn id() -> usize {
        0
    }
}

/// 遅延セグメント木
/// - `(M, ⊕)`, `(Lazy, ∘)`はモノイド。
/// - `∘`(`Lazy`の`op`)は可換な作用素合成。
/// - 準同型性α: `apply(apply(m,l1),l2) = apply(m,l1∘l2)`
/// - 準同型性β: `apply(m1⊕m2,l) = apply(m1,l)⊕apply(m2,l)`
struct SegmentTree<M, Lazy>
where
    M: Monoid,
    Lazy: Monoid,
{
    n: usize,
    height: usize,
    apply: fn(&M::Val, &Lazy::Val) -> M::Val,
    data: Vec<M::Val>,
    lazy: Vec<Lazy::Val>,
}
impl<M, Lazy> SegmentTree<M, Lazy>
where
    M: Monoid,
    Lazy: Monoid, // TODO: 非可換でも実装可能 (https://ikatakos.com/pot/programming_algorithm/data_structure/segment_tree/lazy_segment_tree)
{
    fn new(n: usize, apply: fn(&M::Val, &Lazy::Val) -> M::Val) -> Self {
        let n = n.next_power_of_two();
        let height = (0..62).find(|&x| n == 1 << x).unwrap() + 1;
        Self { n, height, apply, data: vec![M::id(); 2 * n - 1], lazy: vec![Lazy::id(); 2 * n - 1] }
    }
    #[inline(always)]
    fn propagate(&mut self, k: usize) {
        if self.lazy[k] == Lazy::id() {
            return;
        }
        if k < self.n - 1 {
            self.lazy[(k << 1) + 1] = Lazy::op(&self.lazy[(k << 1) + 1], &self.lazy[k]);
            self.lazy[(k << 1) + 2] = Lazy::op(&self.lazy[(k << 1) + 2], &self.lazy[k]);
        }
        self.data[k] = (self.apply)(&self.data[k], &self.lazy[k]);
        self.lazy[k] = Lazy::id();
    }
    #[inline(always)]
    fn propagate_all(&mut self, range: &Range<usize>) {
        let l = range.start + self.n;
        let r = range.end + self.n - 1;
        for d in (0..self.height).rev() {
            self.propagate((l >> d) - 1);
            self.propagate((r >> d) - 1);
        }
    }
    fn query(&mut self, range: Range<usize>) -> M::Val {
        self.propagate_all(&range);
        let mut l = range.start + self.n;
        let mut r = range.end + self.n - 1;
        let mut ans = M::id();
        while l <= r {
            if l & 1 == 1 {
                self.propagate(l - 1);
                ans = M::op(&ans, &self.data[l - 1]);
            }
            if r & 1 == 0 {
                self.propagate(r - 1);
                ans = M::op(&ans, &self.data[r - 1]);
            }
            l = (l + 1) >> 1;
            r = (r - 1) >> 1;
        }
        ans
    }
    fn update(&mut self, range: Range<usize>, val: Lazy::Val) {
        self.propagate_all(&range);
        let mut l = range.start + self.n;
        let mut r = range.end + self.n - 1;
        while l <= r {
            if l & 1 == 1 {
                self.lazy[l - 1] = Lazy::op(&self.lazy[l - 1], &val);
                self.propagate(l - 1);
            }
            if r & 1 == 0 {
                self.lazy[r - 1] = Lazy::op(&self.lazy[r - 1], &val);
                self.propagate(r - 1);
            }
            l = (l + 1) >> 1;
            r = (r - 1) >> 1;
        }
        let l = range.start + self.n;
        let r = range.end + self.n - 1;
        for d in 1..self.height {
            if l & ((1 << d) - 1) != 0 {
                let ll = (l >> d) - 1;
                self.data[ll] = M::op(&self.data[(ll << 1) + 1], &self.data[(ll << 1) + 2]);
            }
            if (r + 1) & ((1 << d) - 1) != 0 {
                let rr = (r >> d) - 1;
                self.data[rr] = M::op(&self.data[(rr << 1) + 1], &self.data[(rr << 1) + 2]);
            }
        }
    }
}
impl<M, Lazy> fmt::Debug for SegmentTree<M, Lazy>
where
    M: Monoid,
    M::Val: fmt::Debug,
    Lazy: Monoid,
    Lazy::Val: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strs =
            self.data.iter().zip(self.lazy.iter()).map(|(d, l)| format!("{:?}/{:?},", d, l)).collect_vec();
        let len = strs.iter().map(|s| s.len()).max().unwrap() + 1;
        for d in 0..self.height {
            for c in strs.iter().take((1 << (d + 1)) - 1).skip((1 << d) - 1) {
                write!(f, "{:width$}", c, width = len << (self.height - 1 - d))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        W: usize,
        N: usize,
        LR: [(Usize1, usize); N],
    }

    let mut st = SegmentTree::<MaxMonoid, MaxMonoid>::new(W, |&x, &y| x.max(y));
    for (l, r) in LR {
        let h = st.query(l..r) + 1;
        st.update(l..r, h);
        // eprintln!("{:?}", st);
        println!("{}", h);
    }
}
