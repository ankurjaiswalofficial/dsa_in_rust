//! K shortest path distances.
//!
//! This implementation computes the `k` shortest distances from a source to every node using a
//! priority queue and per-node best-distance lists.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::k_shortest_paths::k_shortest_distances;
//!
//! let graph = vec![vec![(1, 1), (2, 5)], vec![(2, 1)], vec![]];
//! assert_eq!(k_shortest_distances(&graph, 0, 2)[2], vec![2, 5]);
//! ```

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn k_shortest_distances(graph: &[Vec<(usize, i64)>], start: usize, k: usize) -> Vec<Vec<i64>> {
    let mut best = vec![Vec::<i64>::new(); graph.len()];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0_i64), start));
    while let Some((Reverse(dist), node)) = heap.pop() {
        if best[node].len() >= k {
            continue;
        }
        best[node].push(dist);
        for &(next, weight) in &graph[node] {
            if best[next].len() < k {
                heap.push((Reverse(dist + weight), next));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::k_shortest_distances;

    #[test]
    fn k_shortest_paths_collects_multiple_distances() {
        let graph = vec![vec![(1, 1), (2, 5)], vec![(2, 1)], vec![]];
        assert_eq!(k_shortest_distances(&graph, 0, 2)[2], vec![2, 5]);
    }
}

