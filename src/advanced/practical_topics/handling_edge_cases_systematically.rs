//! Handling edge cases systematically.
//!
//! This module records a reusable checklist for guarding common algorithmic mistakes.
//!
//! # Example
//!
//! ```
//! use project1::advanced::practical_topics::handling_edge_cases_systematically::checklist;
//!
//! assert!(checklist().contains(&"Empty input"));
//! ```

pub fn checklist() -> &'static [&'static str] {
    &["Empty input", "Single element", "Duplicate values", "Overflow boundaries", "Disconnected components"]
}

#[cfg(test)]
mod tests {
    use super::checklist;

    #[test]
    fn edge_case_checklist_mentions_common_failures() {
        assert!(checklist().contains(&"Empty input"));
    }
}

