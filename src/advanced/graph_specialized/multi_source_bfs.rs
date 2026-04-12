//! Multi-source BFS.
//!
//! Multi-source BFS starts from many sources simultaneously. It is the right tool when the nearest
//! source matters more than the source identity.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::multi_source_bfs::multi_source_bfs;
//!
//! let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
//! assert_eq!(multi_source_bfs(&graph, &[0, 3]), vec![0, 1, 1, 0]);
//! ```

use std::collections::VecDeque;

pub fn multi_source_bfs(graph: &[Vec<usize>], sources: &[usize]) -> Vec<i32> {
    let mut dist = vec![-1; graph.len()];
    let mut queue = VecDeque::new();
    for &source in sources {
        dist[source] = 0;
        queue.push_back(source);
    }
    while let Some(node) = queue.pop_front() {
        for &next in &graph[node] {
            if dist[next] == -1 {
                dist[next] = dist[node] + 1;
                queue.push_back(next);
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::multi_source_bfs;

    #[test]
    fn multi_source_bfs_propagates_from_all_sources() {
        let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        assert_eq!(multi_source_bfs(&graph, &[0, 3]), vec![0, 1, 1, 0]);
    }
}

