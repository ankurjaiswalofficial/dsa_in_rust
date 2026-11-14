//! Two-pointer pattern.
//!
//! Two pointers are useful when a sequence's ordering can be exploited from both ends.
//!
//! # Covered topics
//!
//! - Pair sum on a sorted slice with left and right pointers, the usize underflow
//!   of `while left <= right`, and removing duplicates in place with slow and fast
//!   pointers (video: DSA in Rust, episode 12)
//!
//! # Example
//!
//! ```
//! use project1::patterns::two_pointers::{pair_sum, remove_duplicates, two_sum_sorted};
//!
//! assert_eq!(two_sum_sorted(&[1, 2, 4, 7, 11], 9), Some((1, 3)));
//! assert_eq!(pair_sum(&[1, 3, 4, 6, 8, 11], 10), Some((2, 3)));
//!
//! let mut v = vec![1, 1, 2, 2, 2, 3];
//! assert_eq!(remove_duplicates(&mut v), 3);
//! assert_eq!(v, [1, 2, 3]);
//! ```

pub fn two_sum_sorted(values: &[i64], target: i64) -> Option<(usize, usize)> {
    if values.len() < 2 {
        return None;
    }
    let mut left = 0;
    let mut right = values.len() - 1;
    while left < right {
        let sum = values[left] + values[right];
        if sum == target {
            return Some((left, right));
        }
        if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

/// Pair sum on a sorted slice: left starts at the smallest value, right at the
/// largest. Too small moves left up, too big moves right down; each step rules
/// out a whole row of pairs, so it is O(n) time and O(1) space.
///
/// The loop is `while left < right`: with `<=`, both pointers can reach 0 and
/// `right -= 1` underflows the usize ("attempt to subtract with overflow").
/// The input must be sorted; on `[8, 1, 6, 3, 11, 4]` with target 10 it misses 6 + 4.
pub fn pair_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

/// Removes duplicates from a sorted vector in place with a slow `write` pointer
/// and a fast `read` pointer, truncates it, and returns the new length.
/// O(n) time, O(1) extra space (the same job as `Vec::dedup`).
pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut write = 1;
    for read in 1..nums.len() {
        if nums[read] != nums[write - 1] {
            nums[write] = nums[read];
            write += 1;
        }
    }
    nums.truncate(write);
    write
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_pointers_find_pair_in_sorted_array() {
        assert_eq!(two_sum_sorted(&[1, 2, 4, 7, 11], 9), Some((1, 3)));
        assert_eq!(two_sum_sorted(&[1, 3, 5], 100), None);
    }

    #[test]
    fn pair_sum_finds_pairs() {
        let nums = [1, 3, 4, 6, 8, 11];
        assert_eq!(pair_sum(&nums, 10), Some((2, 3)));
        assert_eq!(pair_sum(&nums, 19), Some((4, 5)));
        assert_eq!(pair_sum(&nums, 1), None); // used to underflow with left <= right
        assert_eq!(pair_sum(&nums, 8), None); // 4 + 4 would reuse one number
        assert_eq!(pair_sum(&[], 5), None);
        assert_eq!(pair_sum(&[5], 10), None);
    }

    #[test]
    fn pair_sum_needs_sorted_input() {
        assert_eq!(pair_sum(&[8, 1, 6, 3, 11, 4], 10), None); // 6 + 4 is missed
    }

    #[test]
    fn removes_duplicates_in_place() {
        let mut v = vec![1, 1, 2, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut v), 3);
        assert_eq!(v, [1, 2, 3]);
        let mut empty = Vec::new();
        assert_eq!(remove_duplicates(&mut empty), 0);
        let mut single = vec![7];
        assert_eq!(remove_duplicates(&mut single), 1);
        assert_eq!(single, [7]);
    }
}
