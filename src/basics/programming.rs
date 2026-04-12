//! Programming basics for algorithmic problem solving.
//!
//! This module covers the habits that sit below named data structures and
//! algorithms: variables, control flow, state updates, and input-to-output
//! thinking.
//!
//! # Why it matters
//!
//! Many beginners struggle with DSA because they try to memorize techniques
//! before they can reason about simple loops, counters, and decisions. Strong
//! programming basics make every later topic easier.
//!
//! # Example
//!
//! ```
//! use project1::basics::programming::{count_matches, sum_values};
//!
//! assert_eq!(sum_values(&[1, 2, 3, 4]), 10);
//! assert_eq!(count_matches(&[1, 2, 1, 1], 1), 3);
//! ```

/// Sums a list of values with a simple accumulator loop.
pub fn sum_values(values: &[i32]) -> i32 {
    let mut total = 0;
    for &value in values {
        total += value;
    }
    total
}

/// Counts how many times a target value appears.
pub fn count_matches(values: &[i32], target: i32) -> usize {
    let mut count = 0;
    for &value in values {
        if value == target {
            count += 1;
        }
    }
    count
}

/// Shows the core building blocks most beginner programs rely on.
pub fn programming_summary() -> [&'static str; 5] {
    [
        "Variables store evolving program state.",
        "Conditionals choose between branches.",
        "Loops repeat work over a collection or range.",
        "Functions package logic into reusable units.",
        "Good names and invariants make code explain itself.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_and_counts_work() {
        assert_eq!(sum_values(&[1, 2, 3, 4]), 10);
        assert_eq!(count_matches(&[7, 7, 1, 7], 7), 3);
    }

    #[test]
    fn summary_has_expected_shape() {
        assert_eq!(programming_summary().len(), 5);
    }
}
