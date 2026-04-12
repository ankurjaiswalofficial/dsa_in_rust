//! Fenwick tree, also called a binary indexed tree.
//!
//! It supports prefix-sum queries and point updates in `O(log n)` with linear memory.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::fenwick_tree::FenwickTree;
//!
//! let mut bit = FenwickTree::new(5);
//! bit.add(0, 2);
//! bit.add(1, 3);
//! bit.add(4, 5);
//! assert_eq!(bit.prefix_sum(1), 5);
//! assert_eq!(bit.range_sum(1, 4), 8);
//! ```

#[derive(Debug, Clone)]
pub struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    pub fn add(&mut self, index: usize, delta: i64) {
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }

    pub fn prefix_sum(&self, index: usize) -> i64 {
        let mut i = index + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i &= i - 1;
        }
        sum
    }

    pub fn range_sum(&self, left: usize, right: usize) -> i64 {
        if left > right {
            return 0;
        }
        let right_sum = self.prefix_sum(right);
        let left_sum = if left == 0 { 0 } else { self.prefix_sum(left - 1) };
        right_sum - left_sum
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;

    #[test]
    fn fenwick_supports_prefix_and_range_queries() {
        let mut bit = FenwickTree::new(6);
        for (index, value) in [3, 1, 4, 1, 5, 9].into_iter().enumerate() {
            bit.add(index, value);
        }
        assert_eq!(bit.prefix_sum(2), 8);
        assert_eq!(bit.range_sum(2, 5), 19);
    }
}
