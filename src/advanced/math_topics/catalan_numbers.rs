//! Catalan numbers.
//!
//! Catalan numbers count many recursively structured objects, including:
//! - Valid parenthesizations
//! - Binary-search-tree shapes
//! - Full binary trees
//! - Non-crossing chord matchings
//!
//! This implementation builds them with the standard DP recurrence.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::catalan_numbers::catalan_numbers;
//!
//! assert_eq!(catalan_numbers(5), vec![1, 1, 2, 5, 14, 42]);
//! ```

pub fn catalan_numbers(n: usize) -> Vec<u64> {
    let mut dp = vec![0_u64; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for left in 0..i {
            dp[i] += dp[left] * dp[i - 1 - left];
        }
    }
    dp
}

#[cfg(test)]
mod tests {
    use super::catalan_numbers;

    #[test]
    fn catalan_sequence_is_generated_correctly() {
        assert_eq!(catalan_numbers(5), vec![1, 1, 2, 5, 14, 42]);
    }
}
