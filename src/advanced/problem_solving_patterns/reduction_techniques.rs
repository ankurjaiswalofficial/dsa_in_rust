//! Reduction techniques.
//!
//! Reducing one problem to another lets you reuse a known algorithmic toolbox.
//!
//! # Example
//!
//! ```
//! use project1::advanced::problem_solving_patterns::reduction_techniques::bipartite_matching_reduction;
//!
//! assert!(bipartite_matching_reduction().contains("flow"));
//! ```

pub fn bipartite_matching_reduction() -> &'static str {
    "Maximum bipartite matching reduces to unit-capacity max flow."
}

#[cfg(test)]
mod tests {
    use super::bipartite_matching_reduction;

    #[test]
    fn reduction_summary_mentions_flow() {
        assert!(bipartite_matching_reduction().contains("flow"));
    }
}

