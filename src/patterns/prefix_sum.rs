//! Prefix sum pattern.
//!
//! Prefix sums precompute cumulative totals so range queries become constant time after linear
//! preprocessing.
//!
//! # Covered topics
//!
//! - Range sums with a loop vs a prefix array, the n + 1 layout, i64 sums and 2-D prefix sums
//!   (video: DSA in Rust, episode 10)
//! - Subarray Sum Equals K: brute force vs prefix sums with a HashMap of counts
//!   (video: DSA in Rust, episode 11)
//!
//! # Example
//!
//! ```
//! use project1::patterns::prefix_sum::PrefixSum;
//!
//! let prefix = PrefixSum::new(&[2, 4, 6, 8]);
//! assert_eq!(prefix.range_sum(1, 2), 10);
//! ```
//!
//! ```
//! use project1::patterns::prefix_sum::{build_prefix, range_sum, subarray_sum_count};
//!
//! let prefix = build_prefix(&[3, 1, 4, 1, 5, 9, 2, 6]);
//! assert_eq!(range_sum(&prefix, 2, 5), 19); // 4 + 1 + 5 + 9
//! assert_eq!(subarray_sum_count(&[1, 2, 3], 3), 2); // [1, 2] and [3]
//! ```

use std::collections::HashMap;

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

/// Sum of `a[l..=r]` with a loop: O(n) for every query. Sums in i64, because
/// 100,000 numbers of 100,000 already overflow an i32.
pub fn range_sum_loop(a: &[i32], l: usize, r: usize) -> i64 {
    a[l..=r].iter().map(|&x| x as i64).sum()
}

/// Prefix array with n + 1 entries: `prefix[0] = 0` and `prefix[i]` is the sum
/// of the first `i` numbers. O(n) once.
///
/// The length-n layout (`prefix[i] = a[0] + ... + a[i]`) is the classic
/// off-by-one: `prefix[r] - prefix[l]` drops `a[l]`.
pub fn build_prefix(a: &[i32]) -> Vec<i64> {
    let mut prefix = vec![0; a.len() + 1];
    for i in 0..a.len() {
        prefix[i + 1] = prefix[i] + a[i] as i64;
    }
    prefix
}

/// Sum of `a[l..=r]` from a prefix array built by [`build_prefix`]: O(1).
pub fn range_sum(prefix: &[i64], l: usize, r: usize) -> i64 {
    prefix[r + 1] - prefix[l]
}

/// 2-D prefix sums: `p[i + 1][j + 1]` is the sum of the rectangle from (0, 0)
/// to (i, j), with a border of zeros in row 0 and column 0.
pub fn build_prefix_2d(grid: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let rows = grid.len();
    let cols = grid.first().map_or(0, Vec::len);
    let mut p = vec![vec![0; cols + 1]; rows + 1];
    for i in 0..rows {
        for j in 0..cols {
            p[i + 1][j + 1] = grid[i][j] + p[i][j + 1] + p[i + 1][j] - p[i][j];
        }
    }
    p
}

/// Sum of the rectangle from (r1, c1) to (r2, c2), corners included, in O(1)
/// by inclusion-exclusion.
pub fn rect_sum(p: &[Vec<i64>], r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
    p[r2 + 1][c2 + 1] - p[r1][c2 + 1] - p[r2 + 1][c1] + p[r1][c1]
}

/// Subarray Sum Equals K (LeetCode 560), brute force: every start, every end,
/// with a running sum. O(n^2) time, O(1) space.
pub fn subarray_sum_count_brute(nums: &[i64], k: i64) -> usize {
    let mut count = 0;
    for i in 0..nums.len() {
        let mut sum = 0;
        for &x in &nums[i..] {
            sum += x;
            if sum == k {
                count += 1;
            }
        }
    }
    count
}

/// Subarray Sum Equals K with prefix sums and a HashMap of prefix counts.
/// O(n) average time, O(n) space.
///
/// A subarray ending here sums to k exactly when an earlier prefix equals
/// `prefix - k`. The map starts with `{0: 1}` (the empty prefix), or subarrays
/// that start at index 0 are missed. A sliding window does not work here,
/// because negative numbers can make the sum go down.
pub fn subarray_sum_count(nums: &[i64], k: i64) -> usize {
    let mut seen: HashMap<i64, usize> = HashMap::new();
    seen.insert(0, 1);
    let (mut prefix, mut count) = (0, 0);
    for &x in nums {
        prefix += x;
        if let Some(&c) = seen.get(&(prefix - k)) {
            count += c;
        }
        *seen.entry(prefix).or_insert(0) += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_sum_answers_range_queries() {
        let prefix = PrefixSum::new(&[1, 3, 5, 7]);
        assert_eq!(prefix.range_sum(0, 3), 16);
        assert_eq!(prefix.range_sum(1, 2), 8);
    }

    #[test]
    fn range_sum_matches_the_loop() {
        let a = [3, 1, 4, 1, 5, 9, 2, 6];
        let prefix = build_prefix(&a);
        assert_eq!(prefix, [0, 3, 4, 8, 9, 14, 23, 25, 31]);
        for (l, r) in [(2, 5), (0, 7), (0, 0), (7, 7)] {
            assert_eq!(range_sum(&prefix, l, r), range_sum_loop(&a, l, r));
        }
    }

    #[test]
    fn large_sums_do_not_overflow() {
        let big = vec![100_000; 100_000];
        let prefix = build_prefix(&big);
        assert_eq!(range_sum(&prefix, 0, 99_999), 10_000_000_000);
        assert_eq!(range_sum_loop(&big, 0, 99_999), 10_000_000_000);
    }

    #[test]
    fn rectangle_sums() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let p = build_prefix_2d(&grid);
        assert_eq!(rect_sum(&p, 0, 0, 2, 2), 45);
        assert_eq!(rect_sum(&p, 1, 1, 2, 2), 28);
        assert_eq!(rect_sum(&p, 0, 1, 1, 1), 7);
    }

    #[test]
    fn counts_subarrays_with_sum_k() {
        for f in [subarray_sum_count_brute, subarray_sum_count] {
            assert_eq!(f(&[1, 2, 3], 3), 2);
            assert_eq!(f(&[1, 1, 1], 2), 2);
            assert_eq!(f(&[1, -1, 0], 0), 3);
            assert_eq!(f(&[], 0), 0);
        }
    }

    #[test]
    fn brute_force_and_map_agree() {
        let nums: Vec<i64> = (0..2_000).map(|i| (i * 7 % 11) - 5).collect();
        for k in [-3, 0, 5] {
            assert_eq!(subarray_sum_count_brute(&nums, k), subarray_sum_count(&nums, k));
        }
    }
}
