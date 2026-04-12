//! Exchange argument.
//!
//! An exchange argument proves a greedy choice is safe by showing that any optimal solution can be
//! transformed to use the greedy choice without making the solution worse.
//!
//! # Example
//!
//! ```
//! use project1::advanced::algorithm_design::exchange_argument::activity_selection_exchange_summary;
//!
//! assert!(activity_selection_exchange_summary().contains("earliest finish"));
//! ```

pub fn activity_selection_exchange_summary() -> &'static str {
    "If an optimal schedule does not pick the earliest finishing compatible activity first, swap its first chosen activity with the earliest finish one; feasibility and optimality are preserved."
}

#[cfg(test)]
mod tests {
    use super::activity_selection_exchange_summary;

    #[test]
    fn exchange_argument_summary_mentions_earliest_finish() {
        assert!(activity_selection_exchange_summary().contains("earliest finish"));
    }
}

