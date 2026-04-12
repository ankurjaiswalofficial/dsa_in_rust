//! Exponential search.
//!
//! Exponential search is useful when the array is sorted but we do not initially know a tight
//! upper bound around the target. It doubles the search window until the target is bracketed, then
//! finishes with ordinary binary search.
//!
//! # Example
//!
//! ```
//! use project1::advanced::searching_topics::exponential_search::exponential_search;
//!
//! let data = [2, 4, 8, 16, 32, 64];
//! assert_eq!(exponential_search(&data, 16), Some(3));
//! assert_eq!(exponential_search(&data, 7), None);
//! ```

pub fn exponential_search(values: &[i32], target: i32) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    if values[0] == target {
        return Some(0);
    }
    let mut bound = 1usize;
    while bound < values.len() && values[bound] < target {
        bound *= 2;
    }
    let left = bound / 2;
    let right = values.len().min(bound + 1);
    values[left..right]
        .binary_search(&target)
        .ok()
        .map(|index| left + index)
}

#[cfg(test)]
mod tests {
    use super::exponential_search;

    #[test]
    fn exponential_search_finds_targets_in_sorted_data() {
        let data = [2, 4, 8, 16, 32, 64];
        assert_eq!(exponential_search(&data, 16), Some(3));
        assert_eq!(exponential_search(&data, 7), None);
    }
}
