//! Advanced interval problems.
//!
//! Overlap handling is a recurring interval theme. A common question is how many intervals must be
//! removed to eliminate every overlap.
//!
//! # Example
//!
//! ```
//! use project1::advanced::greedy_topics::interval_problems_advanced::erase_overlap_intervals;
//!
//! let intervals = vec![(1, 2), (2, 3), (3, 4), (1, 3)];
//! assert_eq!(erase_overlap_intervals(&intervals), 1);
//! ```

pub fn erase_overlap_intervals(intervals: &[(i32, i32)]) -> usize {
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|&(_, end)| end);
    let mut removals = 0;
    let mut current_end = i32::MIN;
    for (start, end) in intervals {
        if start < current_end {
            removals += 1;
        } else {
            current_end = end;
        }
    }
    removals
}

#[cfg(test)]
mod tests {
    use super::erase_overlap_intervals;

    #[test]
    fn interval_greedy_counts_required_removals() {
        let intervals = vec![(1, 2), (2, 3), (3, 4), (1, 3)];
        assert_eq!(erase_overlap_intervals(&intervals), 1);
    }
}
