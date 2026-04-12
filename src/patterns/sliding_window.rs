//! Sliding window techniques.
//!
//! Sliding windows are useful when the answer depends on a contiguous region that can be grown or
//! shrunk incrementally.
//!
//! # Example
//!
//! ```
//! use project1::patterns::sliding_window::longest_subarray_with_sum_at_most;
//!
//! assert_eq!(longest_subarray_with_sum_at_most(&[1, 2, 1, 1, 3], 4), 3);
//! ```

pub fn longest_subarray_with_sum_at_most(values: &[i64], limit: i64) -> usize {
    let mut left = 0;
    let mut sum = 0;
    let mut best = 0;
    for right in 0..values.len() {
        sum += values[right];
        while sum > limit && left <= right {
            sum -= values[left];
            left += 1;
        }
        best = best.max(right + 1 - left);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::longest_subarray_with_sum_at_most;

    #[test]
    fn sliding_window_finds_best_length() {
        assert_eq!(longest_subarray_with_sum_at_most(&[1, 2, 1, 1, 3], 4), 3);
        assert_eq!(longest_subarray_with_sum_at_most(&[5, 5, 5], 4), 0);
    }
}
