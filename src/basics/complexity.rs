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

/// Linear scan that counts its comparisons (DSA in Rust, episode 1).
///
/// Worst case (target missing): `n` steps, O(n). Best case (target first): 1 step.
///
/// ```
/// use project1::basics::complexity::linear_scan_steps;
///
/// let items: Vec<u64> = (0..1_000).collect();
/// assert_eq!(linear_scan_steps(&items, 1_000), 1_000); // missing: every item checked
/// assert_eq!(linear_scan_steps(&items, 0), 1); // at the front: one step
/// ```
pub fn linear_scan_steps(items: &[u64], target: u64) -> u64 {
    let mut steps = 0;
    for &x in items {
        steps += 1;
        if x == target {
            break;
        }
    }
    steps
}

/// Pairwise duplicate check that counts its comparisons (DSA in Rust, episode 1).
///
/// Worst case (no duplicate): n(n - 1) / 2 comparisons, which Big-O writes as O(n^2):
/// the constant 1/2 and the lower term -n/2 are dropped.
pub fn has_duplicate_steps(items: &[u64]) -> (bool, u64) {
    let mut steps = 0;
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            steps += 1;
            if items[i] == items[j] {
                return (true, steps);
            }
        }
    }
    (false, steps)
}

/// Best, worst and average step counts of linear search over `0..n`
/// (DSA in Rust, episode 4).
///
/// Best case: the target is first (1 step). Worst case: the target is missing
/// (n steps). Average over every target that is present: (n + 1) / 2 steps.
///
/// ```
/// use project1::basics::complexity::linear_search_cases;
///
/// assert_eq!(linear_search_cases(1_000), (1, 1_000, 500.5));
/// ```
pub fn linear_search_cases(n: u64) -> (u64, u64, f64) {
    let items: Vec<u64> = (0..n).collect();
    let best = linear_scan_steps(&items, 0);
    let worst = linear_scan_steps(&items, n);
    let total: u64 = (0..n).map(|t| linear_scan_steps(&items, t)).sum();
    (best, worst, total as f64 / n as f64)
}

/// Pushes `pushes` items into an empty `Vec` and returns
/// `(reallocations, elements_moved)` (DSA in Rust, episode 4).
///
/// Every reallocation moves the items already stored. Because the capacity
/// doubles, the total moved stays below 2 x the final capacity, so `push` is
/// amortized O(1). Measured with Rust 1.100: 1,000,000 pushes -> 19
/// reallocations and 1,048,572 moves (1.05 per push).
///
/// ```
/// use project1::basics::complexity::vec_push_cost;
///
/// let (reallocations, moved) = vec_push_cost(1_000_000);
/// assert!(reallocations < 30);
/// assert!(moved < 2 * 1_000_000);
/// ```
pub fn vec_push_cost(pushes: usize) -> (usize, u64) {
    let mut v: Vec<usize> = Vec::new();
    let (mut reallocations, mut moved) = (0, 0u64);
    for x in 0..pushes {
        if v.len() == v.capacity() {
            reallocations += 1;
            moved += v.len() as u64;
        }
        v.push(x);
    }
    (reallocations, moved)
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

    #[test]
    fn step_counts_grow_linearly_and_quadratically() {
        for n in [1_000u64, 10_000] {
            let items: Vec<u64> = (0..n).collect();
            assert_eq!(linear_scan_steps(&items, n), n);
            assert_eq!(has_duplicate_steps(&items), (false, n * (n - 1) / 2));
        }
    }

    #[test]
    fn best_case_is_one_step() {
        let mut lucky: Vec<u64> = (0..100_000).collect();
        lucky[1] = 0;
        assert_eq!(linear_scan_steps(&lucky, 0), 1);
        assert_eq!(has_duplicate_steps(&lucky), (true, 1));
    }

    #[test]
    fn linear_search_has_three_costs() {
        assert_eq!(linear_search_cases(1_000), (1, 1_000, 500.5));
        assert_eq!(linear_search_cases(1), (1, 1, 1.0));
    }

    #[test]
    fn vec_push_is_amortized_constant() {
        for n in [10usize, 1_000, 1_000_000] {
            let (reallocations, moved) = vec_push_cost(n);
            assert!(moved < 2 * n as u64, "n = {n}: moved {moved}");
            assert!(reallocations <= (usize::BITS - n.leading_zeros()) as usize);
        }
    }

    #[test]
    fn with_capacity_never_reallocates() {
        let mut v: Vec<u32> = Vec::with_capacity(10_000);
        let start = v.capacity();
        for x in 0..10_000 {
            v.push(x);
        }
        assert_eq!(v.capacity(), start);
    }
}
