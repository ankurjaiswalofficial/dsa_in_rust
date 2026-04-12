//! Randomized algorithms.
//!
//! Randomized algorithms use controlled randomness to simplify logic or improve expected
//! performance. Quickselect is a standard example: choosing a pivot randomly leads to linear
//! expected time for k-th order statistics.
//!
//! # Example
//!
//! ```
//! use project1::advanced::computational_techniques::randomized_algorithms::quickselect;
//!
//! let mut values = [7, 2, 1, 8, 6, 3, 5, 4];
//! assert_eq!(quickselect(&mut values, 3), 4);
//! ```

pub fn quickselect(values: &mut [i32], k: usize) -> i32 {
    let mut left = 0usize;
    let mut right = values.len() - 1;
    loop {
        let pivot_index = left + (right - left) / 2;
        let pivot_index = partition(values, left, right, pivot_index);
        if pivot_index == k {
            return values[pivot_index];
        }
        if pivot_index > k {
            right = pivot_index - 1;
        } else {
            left = pivot_index + 1;
        }
    }
}

fn partition(values: &mut [i32], left: usize, right: usize, pivot_index: usize) -> usize {
    values.swap(pivot_index, right);
    let pivot = values[right];
    let mut store = left;
    for index in left..right {
        if values[index] < pivot {
            values.swap(store, index);
            store += 1;
        }
    }
    values.swap(store, right);
    store
}

#[cfg(test)]
mod tests {
    use super::quickselect;

    #[test]
    fn quickselect_finds_the_kth_smallest() {
        let mut values = [7, 2, 1, 8, 6, 3, 5, 4];
        assert_eq!(quickselect(&mut values, 3), 4);
    }
}

