//! Advanced two-pointer patterns.
//!
//! Two pointers are useful whenever the data is ordered and moving one boundary predictably changes
//! the state of the current answer.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::two_pointer_advanced::pair_with_sum_sorted;
//!
//! assert_eq!(pair_with_sum_sorted(&[1, 2, 4, 7, 11], 15), Some((4, 11)));
//! ```

pub fn pair_with_sum_sorted(values: &[i32], target: i32) -> Option<(i32, i32)> {
    if values.len() < 2 {
        return None;
    }
    let (mut left, mut right) = (0usize, values.len() - 1);
    while left < right {
        let sum = values[left] + values[right];
        if sum == target {
            return Some((values[left], values[right]));
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
    use super::pair_with_sum_sorted;

    #[test]
    fn advanced_two_pointer_finds_a_pair_in_sorted_data() {
        assert_eq!(pair_with_sum_sorted(&[1, 2, 4, 7, 11], 15), Some((4, 11)));
    }
}
