//! Sparse table for immutable range queries.
//!
//! Sparse tables preprocess an array so idempotent range queries such as minimum or maximum can be
//! answered in `O(1)`. The price is that updates are not supported efficiently.
//!
//! This advanced-topic module reuses the shared sparse-table implementation from the crate root.
//!
//! # Example
//!
//! ```
//! use project1::advanced::range_queries::sparse_table::SparseTable;
//!
//! let table = SparseTable::new(&[7, 2, 3, 0, 5, 10, 3, 12, 18]);
//! assert_eq!(table.range_min(1, 4), Some(0));
//! ```

pub use crate::advanced::sparse_table::SparseTable;

#[cfg(test)]
mod tests {
    use super::SparseTable;

    #[test]
    fn sparse_table_answers_static_range_minimum_queries() {
        let table = SparseTable::new(&[7, 2, 3, 0, 5, 10, 3, 12, 18]);
        assert_eq!(table.range_min(1, 4), Some(0));
    }
}
