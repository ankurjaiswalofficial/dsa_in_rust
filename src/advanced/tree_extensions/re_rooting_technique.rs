//! Re-rooting technique.
//!
//! Re-rooting computes a DP value for every possible root by first solving the tree from one root
//! and then transferring the answer across edges.
//!
//! This module reuses the sum-of-distances rerooting implementation from tree DP.
//!
//! # Example
//!
//! ```
//! use project1::advanced::tree_extensions::re_rooting_technique::sum_of_distances_in_tree;
//!
//! let tree = vec![vec![1, 2], vec![0], vec![0]];
//! assert_eq!(sum_of_distances_in_tree(&tree), vec![2, 3, 3]);
//! ```

pub use crate::advanced::dynamic_programming::tree_dp_advanced::sum_of_distances_in_tree;

#[cfg(test)]
mod tests {
    use super::sum_of_distances_in_tree;

    #[test]
    fn rerooting_reexport_runs() {
        let tree = vec![vec![1, 2], vec![0], vec![0]];
        assert_eq!(sum_of_distances_in_tree(&tree), vec![2, 3, 3]);
    }
}

