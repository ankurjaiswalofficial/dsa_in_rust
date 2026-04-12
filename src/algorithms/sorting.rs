//! Sorting algorithms.
//!
//! Sorting highlights the tradeoffs between brute-force thinking and asymptotically efficient
//! divide-and-conquer.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::sorting::{merge_sort, quick_sort};
//!
//! let values = vec![5, 1, 4, 2, 3];
//! assert_eq!(merge_sort(&values), vec![1, 2, 3, 4, 5]);
//! let mut in_place = values.clone();
//! quick_sort(&mut in_place);
//! assert_eq!(in_place, vec![1, 2, 3, 4, 5]);
//! ```

use crate::data_structures::heap::MinHeap;

pub fn bubble_sort<T: Ord>(values: &mut [T]) {
    let n = values.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n.saturating_sub(i + 1) {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn selection_sort<T: Ord>(values: &mut [T]) {
    for i in 0..values.len() {
        let mut best = i;
        for j in i + 1..values.len() {
            if values[j] < values[best] {
                best = j;
            }
        }
        values.swap(i, best);
    }
}

pub fn insertion_sort<T: Ord>(values: &mut [T]) {
    for i in 1..values.len() {
        let mut j = i;
        while j > 0 && values[j] < values[j - 1] {
            values.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    if values.len() <= 1 {
        return values.to_vec();
    }
    let mid = values.len() / 2;
    let left = merge_sort(&values[..mid]);
    let right = merge_sort(&values[mid..]);
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    merged
}

pub fn quick_sort<T: Ord>(values: &mut [T]) {
    if values.len() <= 1 {
        return;
    }
    let pivot = partition(values);
    let (left, right) = values.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(values: &mut [T]) -> usize {
    let pivot_index = values.len() - 1;
    let mut store = 0;
    for i in 0..pivot_index {
        if values[i] <= values[pivot_index] {
            values.swap(i, store);
            store += 1;
        }
    }
    values.swap(store, pivot_index);
    store
}

pub fn counting_sort(values: &[usize]) -> Vec<usize> {
    if values.is_empty() {
        return Vec::new();
    }
    let max_value = *values.iter().max().expect("values is not empty");
    let mut counts = vec![0_usize; max_value + 1];
    for &value in values {
        counts[value] += 1;
    }
    let mut output = Vec::with_capacity(values.len());
    for (value, count) in counts.into_iter().enumerate() {
        for _ in 0..count {
            output.push(value);
        }
    }
    output
}

pub fn radix_sort(values: &[u32]) -> Vec<u32> {
    let mut output = values.to_vec();
    let mut exp = 1;
    let max_value = output.iter().copied().max().unwrap_or(0);
    while exp <= max_value {
        let mut buckets = vec![Vec::new(); 10];
        for &value in &output {
            let digit = ((value / exp) % 10) as usize;
            buckets[digit].push(value);
        }
        output.clear();
        for bucket in buckets {
            output.extend(bucket);
        }
        exp *= 10;
    }
    output
}

pub fn bucket_sort(values: &[f64]) -> Vec<f64> {
    if values.is_empty() {
        return Vec::new();
    }
    let bucket_count = values.len();
    let mut buckets = vec![Vec::new(); bucket_count];
    for &value in values {
        let index = ((value * bucket_count as f64) as usize).min(bucket_count - 1);
        buckets[index].push(value);
    }
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).expect("no NaN values expected"));
    }
    buckets.into_iter().flatten().collect()
}

pub fn heap_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    let mut heap = MinHeap::new();
    for value in values.iter().cloned() {
        heap.push(value);
    }
    let mut sorted = Vec::with_capacity(values.len());
    while let Some(value) = heap.pop() {
        sorted.push(value);
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::{
        bubble_sort, bucket_sort, counting_sort, heap_sort, insertion_sort, merge_sort, quick_sort,
        radix_sort, selection_sort,
    };

    #[test]
    fn sorting_algorithms_handle_duplicates() {
        assert_eq!(merge_sort(&[4, 2, 5, 1, 3]), vec![1, 2, 3, 4, 5]);

        let mut quick = [3, 1, 2, 3, 2];
        quick_sort(&mut quick);
        assert_eq!(quick, [1, 2, 2, 3, 3]);

        let mut bubble = [5, 4, 4, 1];
        bubble_sort(&mut bubble);
        assert_eq!(bubble, [1, 4, 4, 5]);
    }

    #[test]
    fn additional_sorts_work() {
        let mut selection = [4, 3, 2, 1];
        selection_sort(&mut selection);
        assert_eq!(selection, [1, 2, 3, 4]);

        let mut insertion = [9, 7, 8, 6];
        insertion_sort(&mut insertion);
        assert_eq!(insertion, [6, 7, 8, 9]);

        assert_eq!(counting_sort(&[4, 2, 2, 8, 3]), vec![2, 2, 3, 4, 8]);
        assert_eq!(radix_sort(&[170, 45, 75, 90, 802, 24, 2, 66]), vec![2, 24, 45, 66, 75, 90, 170, 802]);
        assert_eq!(heap_sort(&[5, 1, 4, 2]), vec![1, 2, 4, 5]);
        assert_eq!(bucket_sort(&[0.42, 0.32, 0.23, 0.52]), vec![0.23, 0.32, 0.42, 0.52]);
    }
}
