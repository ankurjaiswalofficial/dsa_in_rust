//! Optimization problems via backtracking.
//!
//! Backtracking can solve small optimization problems exactly by exploring every feasible
//! permutation or assignment while maintaining the current best answer.
//!
//! The implementation here solves a tiny travelling-salesperson instance exactly.
//!
//! # Example
//!
//! ```
//! use project1::advanced::backtracking_topics::optimization_problems::tsp_backtracking;
//!
//! let graph = vec![
//!     vec![0, 10, 15, 20],
//!     vec![10, 0, 35, 25],
//!     vec![15, 35, 0, 30],
//!     vec![20, 25, 30, 0],
//! ];
//! assert_eq!(tsp_backtracking(&graph), 80);
//! ```

pub fn tsp_backtracking(graph: &[Vec<i32>]) -> i32 {
    let n = graph.len();
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut best = i32::MAX;
    fn dfs(
        graph: &[Vec<i32>],
        node: usize,
        count: usize,
        cost: i32,
        visited: &mut [bool],
        best: &mut i32,
    ) {
        if cost >= *best {
            return;
        }
        if count == graph.len() {
            *best = (*best).min(cost + graph[node][0]);
            return;
        }
        for next in 0..graph.len() {
            if !visited[next] {
                visited[next] = true;
                dfs(graph, next, count + 1, cost + graph[node][next], visited, best);
                visited[next] = false;
            }
        }
    }
    dfs(graph, 0, 1, 0, &mut visited, &mut best);
    best
}

#[cfg(test)]
mod tests {
    use super::tsp_backtracking;

    #[test]
    fn optimization_backtracking_solves_small_tsp_instances() {
        let graph = vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0],
        ];
        assert_eq!(tsp_backtracking(&graph), 80);
    }
}
