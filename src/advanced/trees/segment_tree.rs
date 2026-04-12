//! Segment tree for range sums with point updates.
//!
//! Segment trees split an array into intervals so that range queries and point updates can both be
//! answered in `O(log n)`. This advanced-topic wrapper points to the reusable implementation that
//! already exists in the crate and documents how to use it from the advanced curriculum.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trees::segment_tree::SegmentTree;
//!
//! let mut tree = SegmentTree::from_slice(&[2, 1, 3, 4]);
//! assert_eq!(tree.query(1, 3), 8);
//! tree.update(2, 10);
//! assert_eq!(tree.query(1, 3), 15);
//! ```

pub use crate::data_structures::segment_tree::SegmentTree;

#[cfg(test)]
mod tests {
    use super::SegmentTree;

    #[test]
    fn advanced_segment_tree_exposes_the_shared_implementation() {
        let mut tree = SegmentTree::from_slice(&[2, 1, 3, 4]);
        assert_eq!(tree.query(0, 2), 6);
        tree.update(1, 9);
        assert_eq!(tree.query(0, 2), 14);
    }
}
