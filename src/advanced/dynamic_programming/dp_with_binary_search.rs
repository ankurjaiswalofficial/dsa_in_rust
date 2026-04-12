//! DP with binary search.
//!
//! Some DP problems need "best previous compatible interval". If the intervals are sorted by end
//! time, binary search finds that compatible predecessor in `O(log n)` and keeps the full DP in
//! `O(n log n)`.
//!
//! This file implements weighted interval scheduling.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::dp_with_binary_search::weighted_interval_scheduling;
//!
//! let jobs = vec![(1, 3, 5), (2, 5, 6), (4, 6, 5), (6, 7, 4)];
//! assert_eq!(weighted_interval_scheduling(&jobs), 14);
//! ```

pub fn weighted_interval_scheduling(jobs: &[(i32, i32, i32)]) -> i32 {
    let mut jobs = jobs.to_vec();
    jobs.sort_by_key(|job| job.1);
    let ends: Vec<_> = jobs.iter().map(|job| job.1).collect();
    let mut dp = vec![0; jobs.len() + 1];
    for i in 1..=jobs.len() {
        let (start, _, profit) = jobs[i - 1];
        let previous = ends.partition_point(|&end| end <= start);
        dp[i] = dp[i - 1].max(dp[previous] + profit);
    }
    dp[jobs.len()]
}

#[cfg(test)]
mod tests {
    use super::weighted_interval_scheduling;

    #[test]
    fn dp_with_binary_search_solves_weighted_interval_scheduling() {
        let jobs = vec![(1, 3, 5), (2, 5, 6), (4, 6, 5), (6, 7, 4)];
        assert_eq!(weighted_interval_scheduling(&jobs), 14);
    }
}
