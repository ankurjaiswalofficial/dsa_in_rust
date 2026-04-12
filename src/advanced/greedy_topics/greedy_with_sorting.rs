//! Greedy with sorting.
//!
//! Many greedy algorithms become obvious only after sorting by the right key. Interval scheduling
//! is the classic example: sorting by finishing time makes the optimal local choice visible.
//!
//! # Example
//!
//! ```
//! use project1::advanced::greedy_topics::greedy_with_sorting::max_non_overlapping_intervals;
//!
//! let intervals = vec![(1, 3), (2, 4), (3, 5), (0, 6)];
//! assert_eq!(max_non_overlapping_intervals(&intervals), 2);
//! ```

pub fn max_non_overlapping_intervals(intervals: &[(i32, i32)]) -> usize {
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|&(_, end)| end);
    let mut answer = 0;
    let mut current_end = i32::MIN;
    for (start, end) in intervals {
        if start >= current_end {
            answer += 1;
            current_end = end;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::max_non_overlapping_intervals;

    #[test]
    fn greedy_with_sorting_solves_interval_scheduling() {
        let intervals = vec![(1, 3), (2, 4), (3, 5), (0, 6)];
        assert_eq!(max_non_overlapping_intervals(&intervals), 2);
    }
}
