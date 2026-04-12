//! Bellman-Ford shortest paths.
//!
//! Bellman-Ford also solves single-source shortest paths, but unlike Dijkstra it can handle
//! negative edge weights. The tradeoff is speed: it relaxes every edge up to `V - 1` times and
//! then performs one more pass to detect negative cycles.
//!
//! This implementation returns `None` when a negative cycle is reachable from the source.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::bellman_ford_algorithm::bellman_ford;
//!
//! let edges = vec![(0, 1, 4), (0, 2, 5), (1, 2, -2), (2, 3, 3)];
//! assert_eq!(bellman_ford(4, &edges, 0), Some(vec![0, 4, 2, 5]));
//! ```

pub fn bellman_ford(node_count: usize, edges: &[(usize, usize, i64)], source: usize) -> Option<Vec<i64>> {
    let mut dist = vec![i64::MAX / 4; node_count];
    dist[source] = 0;
    for _ in 1..node_count {
        let mut changed = false;
        for &(u, v, w) in edges {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for &(u, v, w) in edges {
        if dist[u] + w < dist[v] {
            return None;
        }
    }
    Some(dist)
}

#[cfg(test)]
mod tests {
    use super::bellman_ford;

    #[test]
    fn bellman_ford_handles_negative_edges_without_negative_cycles() {
        let edges = vec![(0, 1, 4), (0, 2, 5), (1, 2, -2), (2, 3, 3)];
        assert_eq!(bellman_ford(4, &edges, 0), Some(vec![0, 4, 2, 5]));
    }

    #[test]
    fn bellman_ford_detects_negative_cycles() {
        let edges = vec![(0, 1, 1), (1, 2, -1), (2, 1, -1)];
        assert_eq!(bellman_ford(3, &edges, 0), None);
    }
}
