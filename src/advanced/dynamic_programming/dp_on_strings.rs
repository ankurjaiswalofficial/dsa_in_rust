//! DP on strings.
//!
//! String DP compares prefixes of two strings or partitions of one string. Edit distance is the
//! canonical example because each state `dp[i][j]` depends on insert, delete, and replace choices.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::dp_on_strings::edit_distance;
//!
//! assert_eq!(edit_distance("kitten", "sitting"), 3);
//! ```

pub fn edit_distance(left: &str, right: &str) -> usize {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let mut dp = vec![vec![0; right.len() + 1]; left.len() + 1];
    for (i, row) in dp.iter_mut().enumerate().take(left.len() + 1) {
        row[0] = i;
    }
    for j in 0..=right.len() {
        dp[0][j] = j;
    }
    for i in 1..=left.len() {
        for j in 1..=right.len() {
            dp[i][j] = if left[i - 1] == right[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1])
            };
        }
    }
    dp[left.len()][right.len()]
}

#[cfg(test)]
mod tests {
    use super::edit_distance;

    #[test]
    fn string_dp_computes_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }
}
