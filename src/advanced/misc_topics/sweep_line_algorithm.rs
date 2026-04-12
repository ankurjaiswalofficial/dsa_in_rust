//! Sweep line algorithm.
//!
//! Sweep line processes geometric or interval events in sorted order while maintaining the active
//! set.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::sweep_line_algorithm::max_overlapping_intervals;
//!
//! let intervals = vec![(1, 3), (2, 5), (4, 6)];
//! assert_eq!(max_overlapping_intervals(&intervals), 2);
//! ```

pub fn max_overlapping_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut events = Vec::with_capacity(intervals.len() * 2);
    for &(left, right) in intervals {
        events.push((left, 1));
        events.push((right, -1));
    }
    events.sort_unstable();
    let mut active = 0;
    let mut best = 0;
    for (_, delta) in events {
        active += delta;
        best = best.max(active);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::max_overlapping_intervals;

    #[test]
    fn sweep_line_counts_maximum_active_intervals() {
        assert_eq!(max_overlapping_intervals(&[(1, 3), (2, 5), (4, 6)]), 2);
    }
}

