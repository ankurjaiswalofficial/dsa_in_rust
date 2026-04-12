//! Mo's algorithm for offline range queries.
//!
//! Mo's algorithm reorders offline queries to minimize how far the active window moves between
//! consecutive queries. That makes it useful when adding or removing one endpoint is cheap, but
//! answering an arbitrary query from scratch is expensive.
//!
//! The example below counts distinct values in many subarrays.
//!
//! # Example
//!
//! ```
//! use project1::advanced::range_queries::mos_algorithm::distinct_values_queries;
//!
//! let values = [1, 2, 1, 3, 2];
//! let queries = [(0, 2), (1, 4)];
//! assert_eq!(distinct_values_queries(&values, &queries), vec![2, 3]);
//! ```

use std::collections::HashMap;

pub fn distinct_values_queries(values: &[i32], queries: &[(usize, usize)]) -> Vec<usize> {
    let block = (values.len() as f64).sqrt() as usize + 1;
    let mut indexed: Vec<_> = queries
        .iter()
        .copied()
        .enumerate()
        .map(|(index, (left, right))| (left, right, index))
        .collect();
    indexed.sort_by_key(|&(left, right, _)| (left / block, right));
    let mut count = HashMap::<i32, usize>::new();
    let mut distinct = 0usize;
    let mut current_left = 0usize;
    let mut current_right = 0usize;
    let mut answer = vec![0; queries.len()];
    for (left, right, index) in indexed {
        while current_right <= right {
            let entry = count.entry(values[current_right]).or_insert(0);
            if *entry == 0 {
                distinct += 1;
            }
            *entry += 1;
            current_right += 1;
        }
        while current_right > right + 1 {
            current_right -= 1;
            let entry = count
                .get_mut(&values[current_right])
                .expect("value should exist in the active window");
            *entry -= 1;
            if *entry == 0 {
                distinct -= 1;
            }
        }
        while current_left < left {
            let entry = count
                .get_mut(&values[current_left])
                .expect("value should exist in the active window");
            *entry -= 1;
            if *entry == 0 {
                distinct -= 1;
            }
            current_left += 1;
        }
        while current_left > left {
            current_left -= 1;
            let entry = count.entry(values[current_left]).or_insert(0);
            if *entry == 0 {
                distinct += 1;
            }
            *entry += 1;
        }
        answer[index] = distinct;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::distinct_values_queries;

    #[test]
    fn mos_algorithm_counts_distinct_values_in_ranges() {
        let values = [1, 2, 1, 3, 2];
        let queries = [(0, 2), (1, 4)];
        assert_eq!(distinct_values_queries(&values, &queries), vec![2, 3]);
    }
}
