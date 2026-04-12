//! Searching algorithms.
//!
//! Binary search is important because it turns sorted order into logarithmic query time.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::searching::{binary_search, lower_bound};
//!
//! let values = [1, 2, 4, 4, 7];
//! assert_eq!(binary_search(&values, &4), Some(2));
//! assert_eq!(lower_bound(&values, &4), 2);
//! ```

pub fn linear_search<T: PartialEq>(values: &[T], target: &T) -> Option<usize> {
    values.iter().position(|value| value == target)
}

pub fn binary_search<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match values[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

pub fn lower_bound<T: Ord>(values: &[T], target: &T) -> usize {
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let mid = left + (right - left) / 2;
        if &values[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

pub fn upper_bound<T: Ord>(values: &[T], target: &T) -> usize {
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let mid = left + (right - left) / 2;
        if &values[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

pub fn first_occurrence<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    let index = lower_bound(values, target);
    values.get(index).is_some_and(|value| value == target).then_some(index)
}

pub fn last_occurrence<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    let index = upper_bound(values, target);
    (index > 0 && &values[index - 1] == target).then_some(index - 1)
}

#[cfg(test)]
mod tests {
    use super::{binary_search, first_occurrence, last_occurrence, linear_search, lower_bound, upper_bound};

    #[test]
    fn searches_find_expected_positions() {
        let values = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&values, &7), Some(3));
        assert_eq!(binary_search(&values, &5), Some(2));
        assert_eq!(binary_search(&values, &6), None);
    }

    #[test]
    fn lower_bound_returns_insertion_point() {
        let values = [1, 2, 4, 4, 6];
        assert_eq!(lower_bound(&values, &4), 2);
        assert_eq!(lower_bound(&values, &5), 4);
        assert_eq!(lower_bound(&values, &0), 0);
        assert_eq!(upper_bound(&values, &4), 4);
        assert_eq!(first_occurrence(&values, &4), Some(2));
        assert_eq!(last_occurrence(&values, &4), Some(3));
    }
}
