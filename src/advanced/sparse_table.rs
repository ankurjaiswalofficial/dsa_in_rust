//! Sparse table for idempotent range minimum queries.
//!
//! Sparse tables trade updates away to answer immutable range queries in `O(1)` after
//! `O(n log n)` preprocessing.
//!
//! # Example
//!
//! ```
//! use project1::advanced::sparse_table::SparseTable;
//!
//! let table = SparseTable::new(&[7, 2, 3, 0, 5, 10, 3, 12, 18]);
//! assert_eq!(table.range_min(1, 4), Some(0));
//! ```

#[derive(Debug, Clone)]
pub struct SparseTable {
    table: Vec<Vec<i64>>,
    log: Vec<usize>,
}

impl SparseTable {
    pub fn new(values: &[i64]) -> Self {
        let n = values.len();
        let mut log = vec![0; n + 1];
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }
        let max_k = if n == 0 { 0 } else { log[n] + 1 };
        let mut table = vec![vec![0; n]; max_k];
        if n > 0 {
            table[0].copy_from_slice(values);
            for k in 1..max_k {
                let len = 1 << k;
                for i in 0..=n - len {
                    table[k][i] = table[k - 1][i].min(table[k - 1][i + (len >> 1)]);
                }
            }
        }
        Self { table, log }
    }

    pub fn range_min(&self, left: usize, right: usize) -> Option<i64> {
        if self.table.is_empty() || left > right {
            return None;
        }
        let len = right - left + 1;
        let k = self.log[len];
        Some(self.table[k][left].min(self.table[k][right + 1 - (1 << k)]))
    }
}

#[cfg(test)]
mod tests {
    use super::SparseTable;

    #[test]
    fn sparse_table_answers_static_range_minimum_queries() {
        let table = SparseTable::new(&[7, 2, 3, 0, 5, 10, 3, 12, 18]);
        assert_eq!(table.range_min(0, 2), Some(2));
        assert_eq!(table.range_min(1, 4), Some(0));
    }
}
