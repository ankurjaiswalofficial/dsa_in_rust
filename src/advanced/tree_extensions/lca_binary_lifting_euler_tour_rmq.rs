//! LCA with binary lifting and Euler tour + RMQ discussion.
//!
//! Lowest common ancestor can be solved by multiple advanced techniques:
//! - Binary lifting
//! - Euler tour + RMQ
//!
//! The executable implementation here reuses the binary-lifting LCA already present in the crate.
//!
//! # Example
//!
//! ```
//! use project1::advanced::tree_extensions::lca_binary_lifting_euler_tour_rmq::LowestCommonAncestor;
//!
//! let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let lca = LowestCommonAncestor::new(&tree, 0);
//! assert_eq!(lca.query(2, 3), 0);
//! ```

pub use crate::advanced::lowest_common_ancestor::LowestCommonAncestor;

#[cfg(test)]
mod tests {
    use super::LowestCommonAncestor;

    #[test]
    fn lca_wrapper_uses_the_existing_binary_lifting_solver() {
        let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let lca = LowestCommonAncestor::new(&tree, 0);
        assert_eq!(lca.query(2, 3), 0);
    }
}
