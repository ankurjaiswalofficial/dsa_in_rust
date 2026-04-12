//! Max flow with Dinic's algorithm.
//!
//! Max flow models how much material can travel from a source to a sink through a capacitated
//! network.
//!
//! # Example
//!
//! ```
//! use project1::advanced::max_flow::Dinic;
//!
//! let mut dinic = Dinic::new(4);
//! dinic.add_edge(0, 1, 3);
//! dinic.add_edge(0, 2, 2);
//! dinic.add_edge(1, 2, 1);
//! dinic.add_edge(1, 3, 2);
//! dinic.add_edge(2, 3, 4);
//! assert_eq!(dinic.max_flow(0, 3), 5);
//! ```

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
}

#[derive(Debug, Clone)]
pub struct Dinic {
    graph: Vec<Vec<Edge>>,
}

impl Dinic {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let forward_rev = self.graph[to].len();
        let backward_rev = self.graph[from].len();
        self.graph[from].push(Edge { to, rev: forward_rev, cap });
        self.graph[to].push(Edge {
            to: from,
            rev: backward_rev,
            cap: 0,
        });
    }

    pub fn max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let n = self.graph.len();
        let mut flow = 0;
        loop {
            let mut level = vec![-1; n];
            let mut queue = VecDeque::from([source]);
            level[source] = 0;
            while let Some(node) = queue.pop_front() {
                for edge in &self.graph[node] {
                    if edge.cap > 0 && level[edge.to] == -1 {
                        level[edge.to] = level[node] + 1;
                        queue.push_back(edge.to);
                    }
                }
            }

            if level[sink] == -1 {
                break;
            }

            let mut it = vec![0; n];
            loop {
                let pushed = self.dfs(source, sink, i64::MAX / 4, &level, &mut it);
                if pushed == 0 {
                    break;
                }
                flow += pushed;
            }
        }
        flow
    }

    fn dfs(
        &mut self,
        node: usize,
        sink: usize,
        pushed: i64,
        level: &[i32],
        it: &mut [usize],
    ) -> i64 {
        if node == sink || pushed == 0 {
            return pushed;
        }

        while it[node] < self.graph[node].len() {
            let edge_index = it[node];
            let edge = self.graph[node][edge_index].clone();
            if edge.cap > 0 && level[edge.to] == level[node] + 1 {
                let flow = self.dfs(edge.to, sink, pushed.min(edge.cap), level, it);
                if flow > 0 {
                    self.graph[node][edge_index].cap -= flow;
                    self.graph[edge.to][edge.rev].cap += flow;
                    return flow;
                }
            }
            it[node] += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Dinic;

    #[test]
    fn dinic_computes_max_flow() {
        let mut dinic = Dinic::new(4);
        dinic.add_edge(0, 1, 3);
        dinic.add_edge(0, 2, 2);
        dinic.add_edge(1, 2, 1);
        dinic.add_edge(1, 3, 2);
        dinic.add_edge(2, 3, 4);
        assert_eq!(dinic.max_flow(0, 3), 5);
    }
}
