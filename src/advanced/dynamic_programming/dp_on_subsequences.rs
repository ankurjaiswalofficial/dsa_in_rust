//! DP on subsequences.
//!
//! Subsequence DP usually decides whether to take or skip the current element. The state often
//! tracks how much of a target sequence has already been formed.
//!
//! The example here counts how many times `target` appears as a subsequence of `source`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::dp_on_subsequences::count_subsequence_matches;
//!
//! assert_eq!(count_subsequence_matches("babgbag", "bag"), 5);
//! ```

pub fn count_subsequence_matches(source: &str, target: &str) -> u64 {
    let source = source.as_bytes();
    let target = target.as_bytes();
    let mut dp = vec![0_u64; target.len() + 1];
    dp[0] = 1;
    for &byte in source {
        for j in (1..=target.len()).rev() {
            if byte == target[j - 1] {
                dp[j] += dp[j - 1];
            }
        }
    }
    dp[target.len()]
}

#[cfg(test)]
mod tests {
    use super::count_subsequence_matches;

    #[test]
    fn subsequence_dp_counts_matches() {
        assert_eq!(count_subsequence_matches("babgbag", "bag"), 5);
    }
}
