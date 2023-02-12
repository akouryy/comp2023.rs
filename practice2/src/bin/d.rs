#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]

use itertools::{iproduct, izip, Itertools};
use petgraph::{prelude::*, stable_graph::IndexType, visit::IntoNodeReferences};
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
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

fn dinic(G: &DiGraph<(), usize, usize>, start: usize, goal: usize) -> (usize, Vec<((usize, usize), usize)>) {
    let nv = G.node_count();
    let mut edge_list = vec![vec![]; nv];
    let mut flow = 0;
    if start < nv && goal < nv {
        for e in G.raw_edges().iter() {
            let (i, j) = (e.source().index(), e.target().index());
            let (eij, eji) = (edge_list[i].len(), edge_list[j].len());
            edge_list[i].push((j, e.weight, eji, Outgoing));
            edge_list[j].push((i, 0, eij, Incoming));
        }
        loop {
            let mut dist = vec![-1; nv];
            dist[start] = 0;
            let mut bfs = std::collections::VecDeque::from(vec![start]);
            while let Some(i) = bfs.pop_front() {
                for &(j, c, _, _) in edge_list[i].iter() {
                    if c > 0 && dist[j] < 0 {
                        dist[j] = dist[i] + 1;
                        bfs.push_back(j);
                    }
                }
            }
            if dist[goal] < 0 {
                break;
            }
            // while(false)
            'flowUpdates: while {
                let mut path = Vec::<(usize, usize)>::new();
                let mut dfs = vec![Some((start, None))];
                while let Some(next) = dfs.pop() {
                    match next {
                        Some((i, from)) => {
                            if let Some((h, ehi)) = from {
                                path.push((h, ehi));
                            }
                            if i == goal {
                                let f = path.iter().map(|&(j, ejk)| edge_list[j][ejk].1).min().unwrap();
                                if f > 0 {
                                    flow += f;
                                    path.iter().for_each(|&(j, ejk)| {
                                        let (k, _, ekj, _) = edge_list[j][ejk];
                                        edge_list[j][ejk].1 -= f;
                                        edge_list[k][ekj].1 += f;
                                    });
                                    continue 'flowUpdates;
                                }
                            }
                            dfs.push(None);
                            for (eij, &(j, c, _, _)) in edge_list[i].iter().enumerate() {
                                if c > 0 && dist[i] < dist[j] {
                                    dfs.push(Some((j, Some((i, eij)))));
                                }
                            }
                        }
                        None => {
                            path.pop();
                        }
                    }
                }
                false
            } {}
        }
    }
    let used_edges = edge_list
        .iter()
        .enumerate()
        .flat_map(|(j, es)| {
            es.iter()
                .flat_map(move |&(i, c, _, dir)| if dir == Incoming && c > 0 { Some(((i, j), c)) } else { None })
        })
        .collect();
    (flow, used_edges)
}

#[deny(dead_code)]
#[proconio::fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [Chars; N],
    }

    let start = N * M;
    let goal = N * M + 1;
    let G = DiGraph::from_edges(iproduct!(0..N, 0..M).flat_map(|(i, j)| {
        if S[i][j] == '#' {
            vec![]
        } else if (i + j) % 2 == 0 {
            let mut a = vec![(start, i * M + j, 1)];
            const D4: [(isize, isize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
            for &(di, dj) in D4.iter() {
                let ii = (i as isize + di) as usize;
                let jj = (j as isize + dj) as usize;
                if ii < N && jj < M {
                    a.push((i * M + j, ii * M + jj, 1));
                }
            }
            a
        } else {
            vec![(i * M + j, goal, 1)]
        }
    }));
    let (flow, used_edges) = dinic(&G, start, goal);
    let mut ans = S;
    for &((u, v), _) in used_edges.iter() {
        if u < start && v < start {
            let ui = u / M;
            let uj = u % M;
            let vi = v / M;
            let vj = v % M;
            if ui == vi {
                ans[ui][min(uj, vj)] = '>';
                ans[ui][max(uj, vj)] = '<';
            } else {
                ans[min(ui, vi)][uj] = 'v';
                ans[max(ui, vi)][uj] = '^';
            }
        }
    }
    println!("{}\n{}", flow, ans.iter().map(|l| l.iter().join("")).join("\n"));
}
