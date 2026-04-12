//! Kosaraju and Tarjan strongly connected components.
//!
//! In a directed graph, a strongly connected component is a maximal set of vertices where every
//! vertex can reach every other one. This file provides two standard algorithms:
//! - `kosaraju_scc`: two DFS passes plus the reversed graph.
//! - `tarjan_scc`: one DFS with discovery times, low-link values, and a stack.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::strongly_connected_components_kosaraju_tarjan::{
//!     kosaraju_scc, tarjan_scc,
//! };
//!
//! let graph = vec![vec![1], vec![2], vec![0, 3], vec![]];
//! assert_eq!(kosaraju_scc(&graph).len(), 2);
//! assert_eq!(tarjan_scc(&graph).len(), 2);
//! ```

pub fn kosaraju_scc(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    fn dfs(node: usize, graph: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
        visited[node] = true;
        for &next in &graph[node] {
            if !visited[next] {
                dfs(next, graph, visited, order);
            }
        }
        order.push(node);
    }

    fn collect(node: usize, graph: &[Vec<usize>], visited: &mut [bool], component: &mut Vec<usize>) {
        visited[node] = true;
        component.push(node);
        for &next in &graph[node] {
            if !visited[next] {
                collect(next, graph, visited, component);
            }
        }
    }

    let n = graph.len();
    let mut reverse = vec![Vec::new(); n];
    for (u, edges) in graph.iter().enumerate() {
        for &v in edges {
            reverse[v].push(u);
        }
    }
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    for node in 0..n {
        if !visited[node] {
            dfs(node, graph, &mut visited, &mut order);
        }
    }
    visited.fill(false);
    let mut sccs = Vec::new();
    while let Some(node) = order.pop() {
        if visited[node] {
            continue;
        }
        let mut component = Vec::new();
        collect(node, &reverse, &mut visited, &mut component);
        sccs.push(component);
    }
    sccs
}

pub fn tarjan_scc(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    struct Tarjan<'a> {
        graph: &'a [Vec<usize>],
        index: usize,
        stack: Vec<usize>,
        on_stack: Vec<bool>,
        tin: Vec<Option<usize>>,
        low: Vec<usize>,
        result: Vec<Vec<usize>>,
    }

    impl<'a> Tarjan<'a> {
        fn dfs(&mut self, node: usize) {
            self.tin[node] = Some(self.index);
            self.low[node] = self.index;
            self.index += 1;
            self.stack.push(node);
            self.on_stack[node] = true;
            for &next in &self.graph[node] {
                if self.tin[next].is_none() {
                    self.dfs(next);
                    self.low[node] = self.low[node].min(self.low[next]);
                } else if self.on_stack[next] {
                    self.low[node] = self.low[node].min(self.tin[next].expect("visited"));
                }
            }
            if self.low[node] == self.tin[node].expect("visited") {
                let mut component = Vec::new();
                while let Some(top) = self.stack.pop() {
                    self.on_stack[top] = false;
                    component.push(top);
                    if top == node {
                        break;
                    }
                }
                self.result.push(component);
            }
        }
    }

    let n = graph.len();
    let mut tarjan = Tarjan {
        graph,
        index: 0,
        stack: Vec::new(),
        on_stack: vec![false; n],
        tin: vec![None; n],
        low: vec![0; n],
        result: Vec::new(),
    };
    for node in 0..n {
        if tarjan.tin[node].is_none() {
            tarjan.dfs(node);
        }
    }
    tarjan.result
}

#[cfg(test)]
mod tests {
    use super::{kosaraju_scc, tarjan_scc};

    #[test]
    fn kosaraju_finds_strongly_connected_components() {
        let graph = vec![vec![1], vec![2], vec![0, 3], vec![]];
        assert_eq!(kosaraju_scc(&graph).len(), 2);
    }

    #[test]
    fn tarjan_finds_strongly_connected_components() {
        let graph = vec![vec![1], vec![2], vec![0, 3], vec![]];
        assert_eq!(tarjan_scc(&graph).len(), 2);
    }
}
