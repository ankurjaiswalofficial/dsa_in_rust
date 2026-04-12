//! Eulerian path for an undirected multigraph using Hierholzer's algorithm.
//!
//! An Eulerian path uses every edge exactly once. For undirected graphs:
//! - An Eulerian circuit exists when every non-isolated vertex has even degree.
//! - An Eulerian path exists when exactly zero or two vertices have odd degree.
//!
//! Hierholzer's algorithm constructs the path by walking unused edges and stitching cycles
//! together.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::eulerian_path_circuit::eulerian_path;
//!
//! let edges = vec![(0, 1), (1, 2), (2, 0)];
//! assert_eq!(eulerian_path(edges.len(), &edges, 3), Some(vec![0, 1, 2, 0]));
//! ```

pub fn eulerian_path(edge_count: usize, edges: &[(usize, usize)], node_count: usize) -> Option<Vec<usize>> {
    let mut degree = vec![0; node_count];
    let mut graph = vec![Vec::<(usize, usize)>::new(); node_count];
    for (id, &(u, v)) in edges.iter().enumerate() {
        degree[u] += 1;
        degree[v] += 1;
        graph[u].push((v, id));
        graph[v].push((u, id));
    }
    let odd: Vec<_> = (0..node_count).filter(|&node| degree[node] % 2 == 1).collect();
    if !odd.is_empty() && odd.len() != 2 {
        return None;
    }
    let start = odd.first().copied().or_else(|| (0..node_count).find(|&node| degree[node] > 0))?;
    let mut used = vec![false; edge_count];
    let mut stack = vec![start];
    let mut path = Vec::new();
    let mut position = vec![0; node_count];
    while let Some(&node) = stack.last() {
        while position[node] < graph[node].len() && used[graph[node][position[node]].1] {
            position[node] += 1;
        }
        if position[node] == graph[node].len() {
            path.push(node);
            stack.pop();
        } else {
            let (next, edge_id) = graph[node][position[node]];
            used[edge_id] = true;
            stack.push(next);
        }
    }
    if path.len() == edge_count + 1 {
        path.reverse();
        Some(path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::eulerian_path;

    #[test]
    fn finds_an_eulerian_circuit_when_one_exists() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        assert_eq!(eulerian_path(edges.len(), &edges, 3), Some(vec![0, 1, 2, 0]));
    }

    #[test]
    fn returns_none_when_degrees_do_not_allow_an_eulerian_path() {
        let edges = vec![(0, 1), (0, 2), (0, 3)];
        assert_eq!(eulerian_path(edges.len(), &edges, 4), None);
    }
}
