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

/// Largest difference between any two numbers, brute force (DSA in Rust,
/// episode 6). Checks every pair: O(n^2) time, O(1) extra space. Slow, but
/// obviously correct, so it is the reference the fast version is tested
/// against. Fewer than two numbers -> `None`; `i64` so `i32::MAX - i32::MIN`
/// fits.
///
/// ```
/// use project1::basics::problem_solving::max_diff_brute;
///
/// assert_eq!(max_diff_brute(&[7, 2, 9, 4]), Some(7));
/// assert_eq!(max_diff_brute(&[5]), None);
/// ```
pub fn max_diff_brute(nums: &[i32]) -> Option<i64> {
    let mut best: Option<i64> = None;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let d = (nums[i] as i64 - nums[j] as i64).abs();
            best = Some(best.map_or(d, |b| b.max(d)));
        }
    }
    best
}

/// Largest difference between any two numbers in one pass: the biggest gap is
/// always max - min. O(n) time, O(1) extra space.
///
/// ```
/// use project1::basics::problem_solving::max_diff_fast;
///
/// assert_eq!(max_diff_fast(&[i32::MIN, i32::MAX]), Some(4_294_967_295));
/// ```
pub fn max_diff_fast(nums: &[i32]) -> Option<i64> {
    if nums.len() < 2 {
        return None;
    }
    let (mut lo, mut hi) = (nums[0], nums[0]);
    for &x in &nums[1..] {
        lo = lo.min(x);
        hi = hi.max(x);
    }
    Some(hi as i64 - lo as i64)
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

    // Tiny xorshift generator so the random check needs no crates.
    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    #[test]
    fn max_diff_edge_cases() {
        for f in [max_diff_brute, max_diff_fast] {
            assert_eq!(f(&[]), None);
            assert_eq!(f(&[5]), None);
            assert_eq!(f(&[3, 3, 3]), Some(0));
            assert_eq!(f(&[-8, -2, -5]), Some(6));
            assert_eq!(f(&[i32::MIN, i32::MAX]), Some(4_294_967_295));
        }
    }

    #[test]
    fn fast_matches_brute_force_on_random_arrays() {
        let mut state = 2026;
        for _ in 0..1_000 {
            let len = (xorshift(&mut state) % 50) as usize;
            let arr: Vec<i32> = (0..len)
                .map(|_| (xorshift(&mut state) % 2001) as i32 - 1000)
                .collect();
            assert_eq!(max_diff_brute(&arr), max_diff_fast(&arr), "{arr:?}");
        }
    }

    #[test]
    fn neighbour_gap_is_a_wrong_idea() {
        let arr: [i32; 3] = [2, 3, 7];
        let neighbours = arr.windows(2).map(|w| (w[0] - w[1]).abs() as i64).max();
        assert_eq!(neighbours, Some(4));
        assert_eq!(max_diff_brute(&arr), Some(5));
    }
}
