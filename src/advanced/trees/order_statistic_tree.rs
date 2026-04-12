//! Order statistic tree built on top of Fenwick frequencies.
//!
//! An order statistic tree augments a balanced-search structure so that rank queries and k-th
//! element queries both remain logarithmic. For learning purposes, this file uses Fenwick-tree
//! frequencies over a bounded value range, which keeps the implementation small while still
//! demonstrating the same core operations.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trees::order_statistic_tree::OrderStatisticTree;
//!
//! let mut tree = OrderStatisticTree::new(10);
//! for value in [2, 4, 4, 7] {
//!     tree.insert(value);
//! }
//!
//! assert_eq!(tree.rank(4), 1);
//! assert_eq!(tree.kth(3), Some(4));
//! ```

#[derive(Debug, Clone)]
pub struct OrderStatisticTree {
    tree: Vec<i64>,
}

impl OrderStatisticTree {
    pub fn new(max_value: usize) -> Self {
        Self {
            tree: vec![0; max_value + 2],
        }
    }

    pub fn insert(&mut self, value: usize) {
        self.add(value, 1);
    }

    pub fn remove(&mut self, value: usize) {
        self.add(value, -1);
    }

    pub fn rank(&self, value: usize) -> i64 {
        self.prefix_sum(value.saturating_sub(1))
    }

    pub fn kth(&self, k: i64) -> Option<usize> {
        if k <= 0 || self.prefix_sum(self.tree.len() - 2) < k {
            return None;
        }
        let mut idx = 0usize;
        let mut bit = 1usize;
        while bit < self.tree.len() {
            bit <<= 1;
        }
        let mut remaining = k;
        let mut step = bit;
        while step > 0 {
            let next = idx + step;
            if next < self.tree.len() && self.tree[next] < remaining {
                idx = next;
                remaining -= self.tree[next];
            }
            step >>= 1;
        }
        Some(idx)
    }

    fn add(&mut self, value: usize, delta: i64) {
        let mut i = value + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }

    fn prefix_sum(&self, value: usize) -> i64 {
        let mut i = (value + 1).min(self.tree.len() - 1);
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i &= i - 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::OrderStatisticTree;

    #[test]
    fn order_statistic_tree_supports_rank_queries() {
        let mut tree = OrderStatisticTree::new(10);
        for value in [2, 4, 4, 7] {
            tree.insert(value);
        }
        assert_eq!(tree.rank(4), 1);
        assert_eq!(tree.rank(8), 4);
    }

    #[test]
    fn order_statistic_tree_supports_kth_queries() {
        let mut tree = OrderStatisticTree::new(10);
        for value in [1, 3, 3, 8] {
            tree.insert(value);
        }
        assert_eq!(tree.kth(1), Some(1));
        assert_eq!(tree.kth(2), Some(3));
        assert_eq!(tree.kth(4), Some(8));
        assert_eq!(tree.kth(5), None);
    }
}
