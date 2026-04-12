//! Time and space complexity notes.
//!
//! Before choosing a structure or algorithm, you need a language for cost.
//! Complexity analysis lets you estimate whether a design will scale.
//!
//! # Example
//!
//! ```
//! use project1::basics::complexity::{ComplexityClass, compare_growth, time_complexity_summary};
//!
//! assert!(compare_growth(ComplexityClass::Linear, ComplexityClass::Quadratic).is_lt());
//! assert!(time_complexity_summary().contains("runtime"));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplexityClass {
    Constant,
    Logarithmic,
    Linear,
    Linearithmic,
    Quadratic,
    Cubic,
    Exponential,
}

impl ComplexityClass {
    pub fn description(self) -> &'static str {
        match self {
            Self::Constant => "O(1): cost does not grow with input size.",
            Self::Logarithmic => "O(log n): shrink the search space each step.",
            Self::Linear => "O(n): one full pass over the input.",
            Self::Linearithmic => "O(n log n): efficient comparison sorting.",
            Self::Quadratic => "O(n^2): nested scanning or brute force.",
            Self::Cubic => "O(n^3): common in naive all-pairs approaches.",
            Self::Exponential => "O(2^n) or worse: usually only for small inputs.",
        }
    }
}

pub fn compare_growth(left: ComplexityClass, right: ComplexityClass) -> std::cmp::Ordering {
    left.cmp(&right)
}

pub fn big_o_summary() -> &'static str {
    "Big O notation describes an asymptotic upper bound on growth as input size increases."
}

pub fn time_complexity_summary() -> &'static str {
    "Time complexity tracks how runtime grows as the input becomes larger."
}

pub fn space_complexity_summary() -> &'static str {
    "Space complexity tracks how much extra memory an algorithm needs beyond the input itself."
}

pub fn amortized_analysis_summary() -> &'static str {
    "Amortized analysis spreads rare expensive operations across many cheap ones."
}

/// Returns a short explanation of best, average, and worst-case analysis.
pub fn case_analysis_summary() -> &'static str {
    "Best case shows the luckiest input, average case models typical inputs, and \
     worst case gives the strongest upper bound when guarantees matter."
}

/// Maps a rough input size to the highest practical brute-force complexity.
///
/// This is intentionally heuristic. It exists to teach planning intuition rather
/// than provide a strict performance law.
pub fn practical_budget_hint(n: usize) -> &'static str {
    match n {
        0..=20 => "Exponential search may still be acceptable.",
        21..=200 => "Quadratic solutions might still pass.",
        201..=100_000 => "Prefer O(n log n) or O(n).",
        _ => "Look for near-linear or logarithmic per-query work.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complexity_order_matches_intuition() {
        assert!(compare_growth(ComplexityClass::Constant, ComplexityClass::Linear).is_lt());
        assert!(compare_growth(ComplexityClass::Linearithmic, ComplexityClass::Quadratic).is_lt());
    }

    #[test]
    fn descriptions_are_present() {
        assert!(ComplexityClass::Logarithmic.description().contains("O(log n)"));
        assert!(big_o_summary().contains("upper bound"));
        assert!(time_complexity_summary().contains("runtime"));
        assert!(space_complexity_summary().contains("memory"));
        assert!(amortized_analysis_summary().contains("Amortized"));
        assert!(case_analysis_summary().contains("worst case"));
        assert!(practical_budget_hint(50_000).contains("O(n log n)"));
    }
}
