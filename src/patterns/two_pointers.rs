//! Two-pointer pattern.
//!
//! Two pointers are useful when a sequence's ordering can be exploited from both ends.
//!
//! # Example
//!
//! ```
//! use project1::patterns::two_pointers::two_sum_sorted;
//!
//! assert_eq!(two_sum_sorted(&[1, 2, 4, 7, 11], 9), Some((1, 3)));
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

#[cfg(test)]
mod tests {
    use super::two_sum_sorted;

    #[test]
    fn two_pointers_find_pair_in_sorted_array() {
        assert_eq!(two_sum_sorted(&[1, 2, 4, 7, 11], 9), Some((1, 3)));
        assert_eq!(two_sum_sorted(&[1, 3, 5], 100), None);
    }
}
