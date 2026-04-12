//! Advanced XOR techniques.
//!
//! XOR is especially powerful because equal values cancel out. That property leads to compact
//! solutions for parity tracking, missing-number recovery, and maximum-XOR problems.
//!
//! The function below computes the maximum XOR of any pair in the input by greedily building the
//! answer from the highest bit down.
//!
//! # Example
//!
//! ```
//! use project1::advanced::bit_manipulation_topics::advanced_xor_techniques::maximum_xor_pair;
//!
//! assert_eq!(maximum_xor_pair(&[3, 10, 5, 25, 2, 8]), 28);
//! ```

use std::collections::HashSet;

pub fn maximum_xor_pair(values: &[i32]) -> i32 {
    let mut answer = 0;
    let mut mask = 0;
    for bit in (0..31).rev() {
        mask |= 1 << bit;
        let prefixes: HashSet<_> = values.iter().map(|&value| value & mask).collect();
        let candidate = answer | (1 << bit);
        if prefixes
            .iter()
            .any(|&prefix| prefixes.contains(&(prefix ^ candidate)))
        {
            answer = candidate;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::maximum_xor_pair;

    #[test]
    fn advanced_xor_techniques_find_the_best_pair() {
        assert_eq!(maximum_xor_pair(&[3, 10, 5, 25, 2, 8]), 28);
    }
}
