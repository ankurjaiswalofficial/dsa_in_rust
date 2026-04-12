//! Meet in the middle.
//!
//! Meet in the middle splits an exponential search space into two halves, enumerates each half,
//! and combines the results faster than full brute force.
//!
//! This module reuses the existing subset-sum style implementation from the miscellaneous topics.
//!
//! # Example
//!
//! ```
//! use project1::advanced::computational_techniques::meet_in_the_middle::max_subset_sum_not_exceeding;
//!
//! assert_eq!(max_subset_sum_not_exceeding(&[3, 34, 4, 12, 5, 2], 10), 10);
//! ```

pub use crate::advanced::misc_topics::meet_in_the_middle::max_subset_sum_not_exceeding;

#[cfg(test)]
mod tests {
    use super::max_subset_sum_not_exceeding;

    #[test]
    fn meet_in_the_middle_reexport_runs() {
        assert_eq!(max_subset_sum_not_exceeding(&[3, 34, 4, 12, 5, 2], 10), 10);
    }
}

