//! Graph algorithms.
//!
//! Graphs model relationships: roads, dependencies, communications, and state transitions.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::graph::{adjacency_list, bfs_order, dijkstra, shortest_path_unweighted, topological_sort};
//!
//! let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
//! assert_eq!(bfs_order(&graph, 0), vec![0, 1, 2, 3]);
//! assert_eq!(topological_sort(&graph), Some(vec![0, 1, 2, 3]));
//! assert_eq!(shortest_path_unweighted(&graph, 0), vec![Some(0), Some(1), Some(1), Some(2)]);
//!
//! let built = adjacency_list(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
//! assert_eq!(built, graph);
//!
//! let weighted = vec![
//!     vec![(1, 4), (2, 1)],
//!     vec![(3, 1)],
//!     vec![(1, 2), (3, 5)],
//!     vec![],
//! ];
//! assert_eq!(dijkstra(&weighted, 0), vec![0, 3, 1, 4]);
//! ```

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

pub fn adjacency_list(node_count: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![Vec::new(); node_count];
    for &(from, to) in edges {
        graph[from].push(to);
    }
    graph
}

pub fn adjacency_matrix(node_count: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut graph = vec![vec![false; node_count]; node_count];
    for &(from, to) in edges {
        graph[from][to] = true;
    }
    graph
}

pub fn bfs_order(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::from([start]);
    let mut order = Vec::new();
    visited[start] = true;

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &graph[node] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }
    order
}

pub fn shortest_path_unweighted(graph: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; graph.len()];
    let mut queue = VecDeque::from([start]);
    dist[start] = Some(0);
    while let Some(node) = queue.pop_front() {
        let current = dist[node].expect("visited node has a distance");
        for &next in &graph[node] {
            if dist[next].is_none() {
                dist[next] = Some(current + 1);
                queue.push_back(next);
            }
        }
    }
    dist
}

pub fn dfs_order(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    fn dfs(node: usize, graph: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
        visited[node] = true;
        order.push(node);
        for &next in &graph[node] {
            if !visited[next] {
                dfs(next, graph, visited, order);
            }
        }
    }

    let mut visited = vec![false; graph.len()];
    let mut order = Vec::new();
    dfs(start, graph, &mut visited, &mut order);
    order
}

pub fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut indegree = vec![0; graph.len()];
    for edges in graph {
        for &to in edges {
            indegree[to] += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (node, &degree) in indegree.iter().enumerate() {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &graph[node] {
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    (order.len() == graph.len()).then_some(order)
}

pub fn dijkstra(graph: &[Vec<(usize, i64)>], start: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX / 4; graph.len()];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((Reverse(0_i64), start));

    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost != dist[node] {
            continue;
        }
        for &(next, weight) in &graph[node] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push((Reverse(next_cost), next));
            }
        }
    }
    dist
}

pub fn connected_components(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    fn dfs(node: usize, graph: &[Vec<usize>], visited: &mut [bool], component: &mut Vec<usize>) {
        visited[node] = true;
        component.push(node);
        for &next in &graph[node] {
            if !visited[next] {
                dfs(next, graph, visited, component);
            }
        }
    }

    let mut visited = vec![false; graph.len()];
    let mut components = Vec::new();
    for node in 0..graph.len() {
        if !visited[node] {
            let mut component = Vec::new();
            dfs(node, graph, &mut visited, &mut component);
            components.push(component);
        }
    }
    components
}

pub fn has_cycle_undirected(graph: &[Vec<usize>]) -> bool {
    fn dfs(node: usize, parent: usize, graph: &[Vec<usize>], visited: &mut [bool]) -> bool {
        visited[node] = true;
        for &next in &graph[node] {
            if !visited[next] {
                if dfs(next, node, graph, visited) {
                    return true;
                }
            } else if next != parent {
                return true;
            }
        }
        false
    }

    let mut visited = vec![false; graph.len()];
    for node in 0..graph.len() {
        if !visited[node] && dfs(node, usize::MAX, graph, &mut visited) {
            return true;
        }
    }
    false
}

pub fn is_bipartite(graph: &[Vec<usize>]) -> bool {
    let mut color = vec![None; graph.len()];
    for start in 0..graph.len() {
        if color[start].is_some() {
            continue;
        }
        let mut queue = VecDeque::from([start]);
        color[start] = Some(0_i32);
        while let Some(node) = queue.pop_front() {
            let current = color[node].expect("node was colored before entering queue");
            for &next in &graph[node] {
                match color[next] {
                    Some(existing) if existing == current => return false,
                    Some(_) => {}
                    None => {
                        color[next] = Some(1 - current);
                        queue.push_back(next);
                    }
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{
        adjacency_list, adjacency_matrix, bfs_order, connected_components, dfs_order, dijkstra,
        has_cycle_undirected, is_bipartite, shortest_path_unweighted, topological_sort,
    };

    #[test]
    fn traversals_visit_reachable_nodes() {
        let graph = vec![vec![1, 2], vec![3], vec![], vec![]];
        assert_eq!(bfs_order(&graph, 0), vec![0, 1, 2, 3]);
        assert_eq!(dfs_order(&graph, 0), vec![0, 1, 3, 2]);
        assert_eq!(shortest_path_unweighted(&graph, 0), vec![Some(0), Some(1), Some(1), Some(2)]);
    }

    #[test]
    fn topological_sort_and_dijkstra_work() {
        let dag = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(topological_sort(&dag), Some(vec![0, 1, 2, 3]));

        let weighted = vec![
            vec![(1, 10), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 8)],
            vec![],
        ];
        assert_eq!(dijkstra(&weighted, 0), vec![0, 3, 1, 4]);
    }

    #[test]
    fn graph_utilities_cover_components_cycles_and_bipartite() {
        let graph = vec![vec![1], vec![0], vec![3], vec![2]];
        assert_eq!(connected_components(&graph), vec![vec![0, 1], vec![2, 3]]);
        assert!(!has_cycle_undirected(&graph));
        assert!(is_bipartite(&graph));

        let cycle_graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert!(has_cycle_undirected(&cycle_graph));
        assert!(!is_bipartite(&cycle_graph));
    }

    #[test]
    fn graph_representations_can_be_built_from_edges() {
        let edges = vec![(0, 1), (0, 2), (1, 3)];
        assert_eq!(adjacency_list(4, &edges), vec![vec![1, 2], vec![3], vec![], vec![]]);
        let matrix = adjacency_matrix(4, &edges);
        assert!(matrix[0][1]);
        assert!(matrix[0][2]);
        assert!(matrix[1][3]);
        assert!(!matrix[2][3]);
    }
}
