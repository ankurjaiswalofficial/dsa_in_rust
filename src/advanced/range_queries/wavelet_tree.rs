//! Wavelet tree.
//!
//! A wavelet tree supports rank, quantile, and range-count queries over static arrays. The core
//! idea is to recursively partition values by a midpoint while storing prefix counts of how many
//! elements go left.
//!
//! # Example
//!
//! ```
//! use project1::advanced::range_queries::wavelet_tree::WaveletTree;
//!
//! let tree = WaveletTree::new(&[5, 1, 2, 6, 3]);
//! assert_eq!(tree.kth_smallest(1, 4, 2), Some(2));
//! assert_eq!(tree.count_less_equal(0, 4, 3), 3);
//! ```

#[derive(Debug, Clone)]
pub struct WaveletTree {
    low: i32,
    high: i32,
    prefix_left: Vec<usize>,
    left: Option<Box<WaveletTree>>,
    right: Option<Box<WaveletTree>>,
}

impl WaveletTree {
    pub fn new(values: &[i32]) -> Self {
        let low = *values.iter().min().unwrap_or(&0);
        let high = *values.iter().max().unwrap_or(&0);
        Self::build(values, low, high)
    }

    fn build(values: &[i32], low: i32, high: i32) -> Self {
        if values.is_empty() || low == high {
            return Self {
                low,
                high,
                prefix_left: vec![0; values.len() + 1],
                left: None,
                right: None,
            };
        }
        let mid = low + (high - low) / 2;
        let mut prefix_left = Vec::with_capacity(values.len() + 1);
        prefix_left.push(0);
        let mut left_values = Vec::new();
        let mut right_values = Vec::new();
        for &value in values {
            if value <= mid {
                left_values.push(value);
            } else {
                right_values.push(value);
            }
            prefix_left.push(left_values.len());
        }
        Self {
            low,
            high,
            prefix_left,
            left: Some(Box::new(Self::build(&left_values, low, mid))),
            right: Some(Box::new(Self::build(&right_values, mid + 1, high))),
        }
    }

    pub fn kth_smallest(&self, left: usize, right: usize, k: usize) -> Option<i32> {
        if left > right || k == 0 || k > right - left + 1 {
            return None;
        }
        Some(self.kth(left + 1, right + 1, k))
    }

    fn kth(&self, left: usize, right: usize, k: usize) -> i32 {
        if self.low == self.high {
            return self.low;
        }
        let in_left = self.prefix_left[right] - self.prefix_left[left - 1];
        let left_before = self.prefix_left[left - 1];
        let left_right = self.prefix_left[right];
        if k <= in_left {
            self.left
                .as_ref()
                .expect("left child")
                .kth(left_before + 1, left_right, k)
        } else {
            let right_before = left - left_before;
            let right_right = right - left_right;
            self.right
                .as_ref()
                .expect("right child")
                .kth(right_before, right_right, k - in_left)
        }
    }

    pub fn count_less_equal(&self, left: usize, right: usize, x: i32) -> usize {
        if left > right {
            return 0;
        }
        self.lte(left + 1, right + 1, x)
    }

    fn lte(&self, left: usize, right: usize, x: i32) -> usize {
        if left > right || x < self.low {
            return 0;
        }
        if self.high <= x {
            return right - left + 1;
        }
        let left_before = self.prefix_left[left - 1];
        let left_right = self.prefix_left[right];
        let left_count = self
            .left
            .as_ref()
            .map_or(0, |tree| tree.lte(left_before + 1, left_right, x));
        let right_before = left - left_before;
        let right_right = right - left_right;
        let right_count = self
            .right
            .as_ref()
            .map_or(0, |tree| tree.lte(right_before, right_right, x));
        left_count + right_count
    }
}

#[cfg(test)]
mod tests {
    use super::WaveletTree;

    #[test]
    fn wavelet_tree_answers_kth_queries() {
        let tree = WaveletTree::new(&[5, 1, 2, 6, 3]);
        assert_eq!(tree.kth_smallest(1, 4, 2), Some(2));
        assert_eq!(tree.kth_smallest(0, 4, 5), Some(6));
    }

    #[test]
    fn wavelet_tree_counts_values_by_threshold() {
        let tree = WaveletTree::new(&[5, 1, 2, 6, 3]);
        assert_eq!(tree.count_less_equal(0, 4, 3), 3);
        assert_eq!(tree.count_less_equal(1, 3, 2), 2);
    }
}
