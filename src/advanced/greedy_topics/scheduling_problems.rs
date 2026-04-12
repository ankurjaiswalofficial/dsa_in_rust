//! Scheduling problems.
//!
//! A useful early scheduling theorem is that ordering jobs by deadline minimizes the maximum
//! lateness when all processing times are fixed and every job is eventually processed.
//!
//! # Example
//!
//! ```
//! use project1::advanced::greedy_topics::scheduling_problems::minimum_maximum_lateness;
//!
//! let jobs = vec![(3, 4), (2, 7), (1, 5)];
//! assert_eq!(minimum_maximum_lateness(&jobs), 0);
//! ```

pub fn minimum_maximum_lateness(jobs: &[(i32, i32)]) -> i32 {
    let mut jobs = jobs.to_vec();
    jobs.sort_by_key(|&(_, deadline)| deadline);
    let mut time = 0;
    let mut lateness = 0;
    for (duration, deadline) in jobs {
        time += duration;
        lateness = lateness.max(time - deadline);
    }
    lateness
}

#[cfg(test)]
mod tests {
    use super::minimum_maximum_lateness;

    #[test]
    fn scheduling_by_deadline_minimizes_maximum_lateness() {
        let jobs = vec![(3, 4), (2, 7), (1, 5)];
        assert_eq!(minimum_maximum_lateness(&jobs), 0);
    }
}
