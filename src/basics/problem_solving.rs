//! Problem-solving fundamentals for DSA work.
//!
//! Algorithms are not only about knowing named techniques. They also require a
//! disciplined way to translate an English problem into states, transitions,
//! invariants, and constraints.
//!
//! # Why this module exists
//!
//! Beginners often jump directly into coding and get lost because the problem
//! was never decomposed clearly. This module introduces a repeatable process:
//!
//! 1. Understand the input and output contract.
//! 2. Identify constraints that rule out brute force.
//! 3. Search for structure such as order, repetition, graph shape, or overlap.
//! 4. State the invariant the algorithm must preserve.
//! 5. Choose the simplest structure that supports the required operations.
//!
//! # Example
//!
//! ```
//! use project1::basics::problem_solving::{classify_constraint_scale, ConstraintScale};
//!
//! assert_eq!(classify_constraint_scale(50), ConstraintScale::Tiny);
//! assert_eq!(classify_constraint_scale(100_000), ConstraintScale::Large);
//! ```

/// Rough buckets for input-size reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintScale {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

/// Categorizes an input size into a rough planning bucket.
///
/// The point is not mathematical precision. The point is to build intuition
/// about what kinds of algorithms are still viable.
pub fn classify_constraint_scale(n: usize) -> ConstraintScale {
    match n {
        0..=100 => ConstraintScale::Tiny,
        101..=10_000 => ConstraintScale::Small,
        10_001..=1_000_000 => ConstraintScale::Large,
        1_000_001..=10_000_000 => ConstraintScale::Huge,
        _ => ConstraintScale::Huge,
    }
}

/// Returns a short checklist for converting a prompt into a technical plan.
pub fn solving_checklist() -> [&'static str; 5] {
    [
        "Write down the exact input, output, and edge cases.",
        "Estimate which time complexities are affordable.",
        "Look for monotonicity, repetition, hierarchy, or graph structure.",
        "Define the invariant each step must preserve.",
        "Pick the smallest data structure that supports the needed operations.",
    ]
}

/// Explains why brute force is still useful during design.
pub fn brute_force_rationale() -> &'static str {
    "A brute-force solution is valuable because it clarifies the exact task, \
     provides a correctness reference, and reveals which operation is too slow."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_classification_is_stable() {
        assert_eq!(classify_constraint_scale(25), ConstraintScale::Tiny);
        assert_eq!(classify_constraint_scale(5_000), ConstraintScale::Small);
        assert_eq!(classify_constraint_scale(100_000), ConstraintScale::Large);
    }

    #[test]
    fn checklist_and_rationale_exist() {
        assert_eq!(solving_checklist().len(), 5);
        assert!(brute_force_rationale().contains("correctness reference"));
    }
}
