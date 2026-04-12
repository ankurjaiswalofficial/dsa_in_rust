//! Fenwick tree, also called a binary indexed tree.
//!
//! A Fenwick tree stores prefix aggregates in a compact array. It is lighter than a segment tree
//! when the problem needs point updates and prefix or range sums. The key bit trick jumps between
//! parent buckets efficiently.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trees::fenwick_tree_binary_indexed_tree::FenwickTree;
//!
//! let mut tree = FenwickTree::new(5);
//! tree.add(0, 2);
//! tree.add(1, 3);
//! tree.add(4, 5);
//! assert_eq!(tree.prefix_sum(1), 5);
//! assert_eq!(tree.range_sum(1, 4), 8);
//! ```

pub use crate::data_structures::fenwick_tree::FenwickTree;

#[cfg(test)]
mod tests {
    use super::FenwickTree;

    #[test]
    fn advanced_fenwick_tree_exposes_prefix_and_range_queries() {
        let mut tree = FenwickTree::new(6);
        for (index, value) in [3, 1, 4, 1, 5, 9].into_iter().enumerate() {
            tree.add(index, value);
        }
        assert_eq!(tree.prefix_sum(2), 8);
        assert_eq!(tree.range_sum(2, 5), 19);
    }
}
