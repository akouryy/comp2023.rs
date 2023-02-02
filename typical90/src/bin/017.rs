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

trait Monoid {
    type Val: Clone;
    fn add(a: &Self::Val, b: &Self::Val) -> Self::Val;
    fn zero() -> Self::Val;
}
struct PlusMonoid<N>
where
    N: Copy,
    N: Num,
{
    _phantom: std::marker::PhantomData<N>,
}
impl<N> Monoid for PlusMonoid<N>
where
    N: Copy,
    N: Num,
{
    type Val = N;
    fn add(a: &N, b: &N) -> N {
        *a + *b
    }
    fn zero() -> N {
        N::zero()
    }
}
struct BIT<M>
where
    M: Monoid,
{
    nodes: Vec<M::Val>,
}
impl<M> BIT<M>
where
    M: Monoid,
{
    fn new(n: usize) -> Self {
        Self { nodes: vec![M::zero(); n] }
    }
    fn add(&mut self, i: usize, x: M::Val) {
        let mut i = i + 1;
        while i <= self.nodes.len() {
            self.nodes[i - 1] = M::add(&self.nodes[i - 1], &x);
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, i: usize) -> M::Val {
        let mut i = i + 1;
        let mut ans = M::zero();
        while i > 0 {
            ans = M::add(&ans, &self.nodes[i - 1]);
            i -= i & i.wrapping_neg();
        }
        ans
    }
}

#[deny(dead_code)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut LR: [(Usize1, Usize1); M],
    }
    LR.sort_by_key(|&(l, r)| (l, !r));

    let mut bit = BIT::<PlusMonoid<usize>>::new(N);
    let mut ans = 0;
    for &(l, r) in LR.iter() {
        ans += bit.sum(r - 1) - bit.sum(l);
        d!(l, r, ans, bit.sum(r - 1), bit.sum(l));
        bit.add(r, 1);
    }
    println!("{}", ans);
}
