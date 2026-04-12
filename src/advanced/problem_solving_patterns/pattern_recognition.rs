//! Pattern recognition.
//!
//! Many problems become manageable only after recognizing the hidden pattern class behind them.
//!
//! # Example
//!
//! ```
//! use project1::advanced::problem_solving_patterns::pattern_recognition::classify_problem;
//!
//! assert_eq!(classify_problem("nearest source in an unweighted graph"), "BFS");
//! ```

pub fn classify_problem(description: &str) -> &'static str {
    if description.contains("unweighted graph") {
        "BFS"
    } else if description.contains("range sum") {
        "Prefix sum"
    } else {
        "Unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::classify_problem;

    #[test]
    fn pattern_recognition_maps_descriptions_to_patterns() {
        assert_eq!(classify_problem("nearest source in an unweighted graph"), "BFS");
    }
}

