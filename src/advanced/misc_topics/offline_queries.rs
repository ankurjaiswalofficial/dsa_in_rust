//! Offline queries.
//!
//! Offline processing means all queries are known in advance, so they can be sorted or grouped to
//! reuse work. The example below answers "how many values in `[l, r]` are greater than `k`?"
//! queries using sorting plus a Fenwick tree.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::offline_queries::count_greater_than_k;
//!
//! let values = [5, 1, 2, 6, 3];
//! let queries = [(1, 3, 1), (0, 4, 4)];
//! assert_eq!(count_greater_than_k(&values, &queries), vec![2, 2]);
//! ```

use crate::data_structures::fenwick_tree::FenwickTree;

pub fn count_greater_than_k(values: &[i64], queries: &[(usize, usize, i64)]) -> Vec<i64> {
    let mut indexed_values: Vec<_> = values.iter().copied().enumerate().collect();
    indexed_values.sort_by_key(|&(_, value)| std::cmp::Reverse(value));
    let mut indexed_queries: Vec<_> = queries
        .iter()
        .copied()
        .enumerate()
        .map(|(index, (left, right, k))| (k, left, right, index))
        .collect();
    indexed_queries.sort_by_key(|&(k, _, _, _)| std::cmp::Reverse(k));
    let mut bit = FenwickTree::new(values.len());
    let mut value_index = 0;
    let mut answer = vec![0; queries.len()];
    for (k, left, right, query_index) in indexed_queries {
        while value_index < indexed_values.len() && indexed_values[value_index].1 > k {
            bit.add(indexed_values[value_index].0, 1);
            value_index += 1;
        }
        answer[query_index] = bit.range_sum(left, right);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::count_greater_than_k;

    #[test]
    fn offline_queries_answer_greater_than_threshold_counts() {
        let values = [5, 1, 2, 6, 3];
        let queries = [(1, 3, 1), (0, 4, 4)];
        assert_eq!(count_greater_than_k(&values, &queries), vec![2, 2]);
    }
}
