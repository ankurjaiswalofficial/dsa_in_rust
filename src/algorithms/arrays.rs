//! Array algorithms and common interview patterns.
//!
//! Arrays are the default sequential container in most problems because they
//! provide contiguous storage and constant-time indexing.
//!
//! # Covered topics
//!
//! - Traversal
//! - Insertion / deletion
//! - Searching
//! - Prefix sum
//! - Sliding window
//! - Two pointers
//! - Kadane's algorithm
//! - Merge intervals
//! - Rotate array
//! - Subarrays
//!
//! # Example
//!
//! ```
//! use project1::algorithms::arrays::{kadane_max_subarray_sum, rotate_right};
//!
//! assert_eq!(kadane_max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
//! assert_eq!(rotate_right(&[1, 2, 3, 4], 1), vec![4, 1, 2, 3]);
//! ```

use crate::algorithms::searching::linear_search;
use crate::patterns::prefix_sum::PrefixSum;
use crate::patterns::sliding_window::longest_subarray_with_sum_at_most;
use crate::patterns::two_pointers::two_sum_sorted;

pub fn traverse(values: &[i32]) -> Vec<i32> {
    values.to_vec()
}

pub fn insert_at(values: &[i32], index: usize, value: i32) -> Vec<i32> {
    let mut result = values.to_vec();
    result.insert(index, value);
    result
}

pub fn delete_at(values: &[i32], index: usize) -> Vec<i32> {
    let mut result = values.to_vec();
    if index < result.len() {
        result.remove(index);
    }
    result
}

pub fn search(values: &[i32], target: i32) -> Option<usize> {
    linear_search(values, &target)
}

pub fn prefix_sum_range(values: &[i64], left: usize, right: usize) -> i64 {
    PrefixSum::new(values).range_sum(left, right)
}

pub fn longest_window_with_sum_at_most(values: &[i64], limit: i64) -> usize {
    longest_subarray_with_sum_at_most(values, limit)
}

pub fn two_sum_on_sorted(values: &[i64], target: i64) -> Option<(usize, usize)> {
    two_sum_sorted(values, target)
}

pub fn kadane_max_subarray_sum(values: &[i32]) -> Option<i32> {
    let mut iter = values.iter().copied();
    let first = iter.next()?;
    let mut best = first;
    let mut current = first;
    for value in iter {
        current = value.max(current + value);
        best = best.max(current);
    }
    Some(best)
}

pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return Vec::new();
    }
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|&(start, _)| start);

    let mut merged = vec![intervals[0]];
    for (start, end) in intervals.into_iter().skip(1) {
        let last = merged.last_mut().expect("merged is never empty here");
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

pub fn rotate_right(values: &[i32], k: usize) -> Vec<i32> {
    if values.is_empty() {
        return Vec::new();
    }
    let n = values.len();
    let k = k % n;
    let mut result = values.to_vec();
    result.rotate_right(k);
    result
}

pub fn all_subarray_sums(values: &[i32]) -> Vec<i32> {
    let mut sums = Vec::new();
    for start in 0..values.len() {
        let mut running = 0;
        for &value in &values[start..] {
            running += value;
            sums.push(running);
        }
    }
    sums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_basics_work() {
        assert_eq!(traverse(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(insert_at(&[1, 3], 1, 2), vec![1, 2, 3]);
        assert_eq!(delete_at(&[1, 2, 3], 1), vec![1, 3]);
        assert_eq!(search(&[4, 5, 6], 5), Some(1));
    }

    #[test]
    fn array_patterns_are_covered() {
        assert_eq!(prefix_sum_range(&[1, 2, 3, 4], 1, 3), 9);
        assert_eq!(longest_window_with_sum_at_most(&[1, 2, 1, 1, 3], 4), 3);
        assert_eq!(two_sum_on_sorted(&[1, 2, 4, 7], 9), Some((1, 3)));
        assert_eq!(kadane_max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
    }

    #[test]
    fn interval_rotation_and_subarrays_work() {
        assert_eq!(merge_intervals(&[(1, 3), (2, 6), (8, 10)]), vec![(1, 6), (8, 10)]);
        assert_eq!(rotate_right(&[1, 2, 3, 4], 1), vec![4, 1, 2, 3]);
        assert_eq!(all_subarray_sums(&[1, 2]), vec![1, 3, 2]);
    }
}
