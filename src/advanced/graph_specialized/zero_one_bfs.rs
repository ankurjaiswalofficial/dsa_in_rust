//! 0-1 BFS.
//!
//! 0-1 BFS solves shortest paths when every edge weight is either `0` or `1`. A deque replaces the
//! priority queue from Dijkstra: zero-weight relaxations go to the front, one-weight relaxations go
//! to the back.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::zero_one_bfs::zero_one_bfs;
//!
//! let graph = vec![vec![(1, 0), (2, 1)], vec![(2, 1)], vec![]];
//! assert_eq!(zero_one_bfs(&graph, 0), vec![0, 0, 1]);
//! ```

use std::collections::VecDeque;

pub fn zero_one_bfs(graph: &[Vec<(usize, i32)>], start: usize) -> Vec<i32> {
    let mut dist = vec![i32::MAX / 4; graph.len()];
    let mut deque = VecDeque::from([start]);
    dist[start] = 0;
    while let Some(node) = deque.pop_front() {
        for &(next, weight) in &graph[node] {
            let candidate = dist[node] + weight;
            if candidate < dist[next] {
                dist[next] = candidate;
                if weight == 0 {
                    deque.push_front(next);
                } else {
                    deque.push_back(next);
                }
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::zero_one_bfs;

    #[test]
    fn zero_one_bfs_computes_shortest_paths() {
        let graph = vec![vec![(1, 0), (2, 1)], vec![(2, 1)], vec![]];
        assert_eq!(zero_one_bfs(&graph, 0), vec![0, 0, 1]);
    }
}

