//! Combinatorics: nCr and nPr.
//!
//! Counting formulas show up everywhere in dynamic programming, probability, and discrete math.
//! This file implements direct multiplicative versions of combinations and permutations.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::combinatorics_ncr_npr::{ncr, npr};
//!
//! assert_eq!(ncr(5, 2), 10);
//! assert_eq!(npr(5, 2), 20);
//! ```

pub fn ncr(n: u64, r: u64) -> u64 {
    let r = r.min(n - r);
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..r {
        numerator *= n - i;
        denominator *= i + 1;
    }
    numerator / denominator
}

pub fn npr(n: u64, r: u64) -> u64 {
    let mut result = 1;
    for i in 0..r {
        result *= n - i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{ncr, npr};

    #[test]
    fn combinations_and_permutations_are_computed_correctly() {
        assert_eq!(ncr(5, 2), 10);
        assert_eq!(npr(5, 2), 20);
    }
}
