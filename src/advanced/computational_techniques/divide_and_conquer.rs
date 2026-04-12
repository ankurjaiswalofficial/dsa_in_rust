//! Divide and conquer.
//!
//! Divide and conquer breaks a problem into smaller independent subproblems, solves them
//! recursively, and combines the answers. Merge sort is the canonical example because the split and
//! merge steps are both explicit.
//!
//! # Example
//!
//! ```
//! use project1::advanced::computational_techniques::divide_and_conquer::merge_sort;
//!
//! let mut values = [5, 2, 4, 1, 3];
//! merge_sort(&mut values);
//! assert_eq!(values, [1, 2, 3, 4, 5]);
//! ```

pub fn merge_sort(values: &mut [i32]) {
    if values.len() <= 1 {
        return;
    }
    let mid = values.len() / 2;
    merge_sort(&mut values[..mid]);
    merge_sort(&mut values[mid..]);
    let mut merged = values.to_vec();
    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < values.len() {
        if values[i] <= values[j] {
            merged[k] = values[i];
            i += 1;
        } else {
            merged[k] = values[j];
            j += 1;
        }
        k += 1;
    }
    while i < mid {
        merged[k] = values[i];
        i += 1;
        k += 1;
    }
    while j < values.len() {
        merged[k] = values[j];
        j += 1;
        k += 1;
    }
    values.copy_from_slice(&merged);
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn divide_and_conquer_merge_sort_works() {
        let mut values = [5, 2, 4, 1, 3];
        merge_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }
}

