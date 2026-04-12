//! Dijkstra shortest-path algorithm.
//!
//! Dijkstra's algorithm solves the single-source shortest-path problem on graphs with
//! non-negative edge weights. It repeatedly takes the currently closest unprocessed node from a
//! min-priority queue and relaxes its outgoing edges.
//!
//! This module reuses the main implementation from `algorithms::graph` so the advanced section can
//! document the topic without duplicating logic.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::dijkstra_algorithm::dijkstra;
//!
//! let graph = vec![
//!     vec![(1, 4), (2, 1)],
//!     vec![(3, 1)],
//!     vec![(1, 2), (3, 5)],
//!     vec![],
//! ];
//! assert_eq!(dijkstra(&graph, 0), vec![0, 3, 1, 4]);
//! ```

pub use crate::algorithms::graph::dijkstra;

#[cfg(test)]
mod tests {
    use super::dijkstra;

    #[test]
    fn dijkstra_computes_shortest_paths() {
        let graph = vec![
            vec![(1, 4), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 5)],
            vec![],
        ];
        assert_eq!(dijkstra(&graph, 0), vec![0, 3, 1, 4]);
    }
}
