//! Prefix sum pattern.
//!
//! Prefix sums precompute cumulative totals so range queries become constant time after linear
//! preprocessing.
//!
//! # Example
//!
//! ```
//! use project1::patterns::prefix_sum::PrefixSum;
//!
//! let prefix = PrefixSum::new(&[2, 4, 6, 8]);
//! assert_eq!(prefix.range_sum(1, 2), 10);
//! ```

#[derive(Debug, Clone)]
pub struct PrefixSum {
    prefix: Vec<i64>,
}

impl PrefixSum {
    pub fn new(values: &[i64]) -> Self {
        let mut prefix = vec![0; values.len() + 1];
        for (index, &value) in values.iter().enumerate() {
            prefix[index + 1] = prefix[index] + value;
        }
        Self { prefix }
    }

    pub fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.prefix[right + 1] - self.prefix[left]
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixSum;

    #[test]
    fn prefix_sum_answers_range_queries() {
        let prefix = PrefixSum::new(&[1, 3, 5, 7]);
        assert_eq!(prefix.range_sum(0, 3), 16);
        assert_eq!(prefix.range_sum(1, 2), 8);
    }
}
