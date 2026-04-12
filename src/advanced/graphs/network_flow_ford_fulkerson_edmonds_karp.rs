//! Edmonds-Karp maximum flow.
//!
//! Maximum-flow algorithms repeatedly find augmenting paths in the residual graph and push flow
//! along them. Edmonds-Karp is the BFS-based specialization of Ford-Fulkerson, which guarantees
//! polynomial time by always choosing a shortest augmenting path in edge count.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::network_flow_ford_fulkerson_edmonds_karp::edmonds_karp;
//!
//! let capacity = vec![
//!     vec![0, 3, 2, 0],
//!     vec![0, 0, 1, 2],
//!     vec![0, 0, 0, 4],
//!     vec![0, 0, 0, 0],
//! ];
//! assert_eq!(edmonds_karp(capacity, 0, 3), 5);
//! ```

use std::collections::VecDeque;

pub fn edmonds_karp(capacity: Vec<Vec<i64>>, source: usize, sink: usize) -> i64 {
    let n = capacity.len();
    let mut residual = capacity;
    let mut max_flow = 0;
    loop {
        let mut parent = vec![usize::MAX; n];
        let mut queue = VecDeque::from([source]);
        parent[source] = source;
        while let Some(node) = queue.pop_front() {
            for next in 0..n {
                if parent[next] == usize::MAX && residual[node][next] > 0 {
                    parent[next] = node;
                    queue.push_back(next);
                }
            }
        }
        if parent[sink] == usize::MAX {
            break;
        }
        let mut flow = i64::MAX;
        let mut node = sink;
        while node != source {
            let prev = parent[node];
            flow = flow.min(residual[prev][node]);
            node = prev;
        }
        node = sink;
        while node != source {
            let prev = parent[node];
            residual[prev][node] -= flow;
            residual[node][prev] += flow;
            node = prev;
        }
        max_flow += flow;
    }
    max_flow
}

#[cfg(test)]
mod tests {
    use super::edmonds_karp;

    #[test]
    fn edmonds_karp_computes_max_flow() {
        let capacity = vec![
            vec![0, 3, 2, 0],
            vec![0, 0, 1, 2],
            vec![0, 0, 0, 4],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(edmonds_karp(capacity, 0, 3), 5);
    }
}
