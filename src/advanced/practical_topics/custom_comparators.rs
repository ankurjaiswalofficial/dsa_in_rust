//! Custom comparators.
//!
//! Custom comparators let algorithms sort by problem-specific priorities rather than natural order.
//!
//! # Example
//!
//! ```
//! use project1::advanced::practical_topics::custom_comparators::sort_pairs_by_second_then_first;
//!
//! let mut pairs = vec![(2, 1), (1, 1), (3, 0)];
//! sort_pairs_by_second_then_first(&mut pairs);
//! assert_eq!(pairs, vec![(3, 0), (1, 1), (2, 1)]);
//! ```

pub fn sort_pairs_by_second_then_first(values: &mut [(i32, i32)]) {
    values.sort_by_key(|&(first, second)| (second, first));
}

#[cfg(test)]
mod tests {
    use super::sort_pairs_by_second_then_first;

    #[test]
    fn custom_comparator_sorts_by_multiple_keys() {
        let mut pairs = vec![(2, 1), (1, 1), (3, 0)];
        sort_pairs_by_second_then_first(&mut pairs);
        assert_eq!(pairs, vec![(3, 0), (1, 1), (2, 1)]);
    }
}

