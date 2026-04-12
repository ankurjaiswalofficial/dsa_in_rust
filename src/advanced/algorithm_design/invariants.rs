//! Invariants.
//!
//! An invariant is a property that remains true throughout an algorithm. Good invariants explain
//! why the algorithm is correct, not just how it executes.
//!
//! # Example
//!
//! ```
//! use project1::advanced::algorithm_design::invariants::binary_search_invariant;
//!
//! assert!(binary_search_invariant().contains("target"));
//! ```

pub fn binary_search_invariant() -> &'static str {
    "If the target exists, it always lies within the current search interval."
}

#[cfg(test)]
mod tests {
    use super::binary_search_invariant;

    #[test]
    fn invariant_summary_mentions_the_target() {
        assert!(binary_search_invariant().contains("target"));
    }
}

