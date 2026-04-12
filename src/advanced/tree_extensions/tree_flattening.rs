//! Tree flattening.
//!
//! Tree flattening maps each subtree to a contiguous range by DFS order. This is the bridge
//! between tree problems and array-based range-query structures.
//!
//! # Example
//!
//! ```
//! use project1::advanced::tree_extensions::tree_flattening::flatten_tree;
//!
//! let tree = vec![vec![1, 2], vec![0], vec![0]];
//! let (start, end, order) = flatten_tree(&tree, 0);
//! assert_eq!(order.len(), 3);
//! assert_eq!(start[0], 0);
//! assert_eq!(end[0], 3);
//! ```

pub use crate::advanced::tree_extensions::euler_tour_technique::euler_tour as flatten_tree;

#[cfg(test)]
mod tests {
    use super::flatten_tree;

    #[test]
    fn tree_flattening_reuses_euler_tour_ranges() {
        let tree = vec![vec![1, 2], vec![0], vec![0]];
        let (start, end, order) = flatten_tree(&tree, 0);
        assert_eq!(order.len(), 3);
        assert_eq!(start[0], 0);
        assert_eq!(end[0], 3);
    }
}

