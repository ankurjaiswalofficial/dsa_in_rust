//! Backtracking vs branch and bound.
//!
//! Backtracking explores the search space and prunes states that violate feasibility constraints.
//! Branch and bound goes one step further: it also prunes feasible partial states when a bound
//! proves they cannot beat the current best answer.
//!
//! # Example
//!
//! ```
//! use project1::advanced::computational_techniques::backtracking_vs_branch_and_bound::distinction;
//!
//! assert!(distinction().contains("bound"));
//! ```

pub fn distinction() -> &'static str {
    "Backtracking prunes infeasible states; branch and bound also prunes feasible states whose bound cannot improve the incumbent."
}

#[cfg(test)]
mod tests {
    use super::distinction;

    #[test]
    fn distinction_mentions_the_bound() {
        assert!(distinction().contains("bound"));
    }
}

