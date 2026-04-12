//! Transformations such as array to graph.
//!
//! A useful competitive-programming skill is recasting a structure into another one that exposes a
//! standard algorithm more clearly.
//!
//! # Example
//!
//! ```
//! use project1::advanced::problem_solving_patterns::transformations_array_to_graph::array_jump_graph;
//!
//! assert_eq!(array_jump_graph(&[2, 1, 0]), vec![vec![1, 2], vec![2], vec![]]);
//! ```

pub fn array_jump_graph(jumps: &[usize]) -> Vec<Vec<usize>> {
    let mut graph = vec![Vec::new(); jumps.len()];
    for i in 0..jumps.len() {
        for next in i + 1..=usize::min(i + jumps[i], jumps.len().saturating_sub(1)) {
            graph[i].push(next);
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::array_jump_graph;

    #[test]
    fn transformation_builds_a_graph_from_jump_lengths() {
        assert_eq!(array_jump_graph(&[2, 1, 0]), vec![vec![1, 2], vec![2], vec![]]);
    }
}

