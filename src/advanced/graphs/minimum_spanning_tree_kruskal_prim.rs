//! Kruskal and Prim minimum spanning tree implementations.
//!
//! A minimum spanning tree connects all vertices of a weighted undirected graph with minimum total
//! edge weight.
//!
//! This file includes two classic approaches:
//! - `kruskal`: sort edges and use union-find to skip cycle-forming edges.
//! - `prim`: grow the tree one frontier edge at a time using a heap.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::minimum_spanning_tree_kruskal_prim::{kruskal, prim};
//!
//! let edges = vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)];
//! assert_eq!(kruskal(3, &edges).0, 3);
//!
//! let graph = vec![
//!     vec![(1, 1), (2, 3)],
//!     vec![(0, 1), (2, 2)],
//!     vec![(0, 3), (1, 2)],
//! ];
//! assert_eq!(prim(&graph, 0).0, 3);
//! ```

use crate::data_structures::disjoint_set::DisjointSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn kruskal(node_count: usize, edges: &[(usize, usize, i64)]) -> (i64, Vec<(usize, usize, i64)>) {
    let mut edges = edges.to_vec();
    edges.sort_by_key(|edge| edge.2);
    let mut dsu = DisjointSet::new(node_count);
    let mut cost = 0;
    let mut chosen = Vec::new();
    for (u, v, w) in edges {
        if dsu.union(u, v) {
            cost += w;
            chosen.push((u, v, w));
        }
    }
    (cost, chosen)
}

pub fn prim(graph: &[Vec<(usize, i64)>], start: usize) -> (i64, Vec<(usize, usize, i64)>) {
    let mut visited = vec![false; graph.len()];
    let mut heap = BinaryHeap::new();
    let mut cost = 0;
    let mut chosen = Vec::new();
    visited[start] = true;
    for &(to, weight) in &graph[start] {
        heap.push((Reverse(weight), start, to));
    }
    while let Some((Reverse(weight), from, to)) = heap.pop() {
        if visited[to] {
            continue;
        }
        visited[to] = true;
        cost += weight;
        chosen.push((from, to, weight));
        for &(next, next_weight) in &graph[to] {
            if !visited[next] {
                heap.push((Reverse(next_weight), to, next));
            }
        }
    }
    (cost, chosen)
}

#[cfg(test)]
mod tests {
    use super::{kruskal, prim};

    #[test]
    fn kruskal_builds_a_minimum_spanning_tree() {
        let edges = vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)];
        let (cost, chosen) = kruskal(3, &edges);
        assert_eq!(cost, 3);
        assert_eq!(chosen.len(), 2);
    }

    #[test]
    fn prim_builds_a_minimum_spanning_tree() {
        let graph = vec![
            vec![(1, 1), (2, 3)],
            vec![(0, 1), (2, 2)],
            vec![(0, 3), (1, 2)],
        ];
        let (cost, chosen) = prim(&graph, 0);
        assert_eq!(cost, 3);
        assert_eq!(chosen.len(), 2);
    }
}
