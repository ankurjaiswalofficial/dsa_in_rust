//! Digit DP.
//!
//! Digit DP processes a number one digit at a time under an upper-bound constraint. Typical state
//! dimensions include current position, whether the prefix is still tight, and any property we are
//! tracking.
//!
//! The implementation here counts how many numbers in `[0, limit]` have no equal adjacent digits.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::digit_dp::count_without_adjacent_equals;
//!
//! assert_eq!(count_without_adjacent_equals(25), 24);
//! ```

pub fn count_without_adjacent_equals(limit: u64) -> u64 {
    let digits: Vec<u8> = limit.to_string().bytes().map(|b| b - b'0').collect();
    let mut memo = std::collections::HashMap::new();
    fn dfs(
        pos: usize,
        prev: i32,
        tight: bool,
        started: bool,
        digits: &[u8],
        memo: &mut std::collections::HashMap<(usize, i32, bool, bool), u64>,
    ) -> u64 {
        if pos == digits.len() {
            return 1;
        }
        let key = (pos, prev, tight, started);
        if !tight {
            if let Some(&value) = memo.get(&key) {
                return value;
            }
        }
        let mut answer = 0;
        let limit = if tight { digits[pos] } else { 9 };
        for digit in 0..=limit {
            let next_started = started || digit != 0;
            if next_started && started && prev == digit as i32 {
                continue;
            }
            answer += dfs(
                pos + 1,
                if next_started { digit as i32 } else { -1 },
                tight && digit == limit,
                next_started,
                digits,
                memo,
            );
        }
        if !tight {
            memo.insert(key, answer);
        }
        answer
    }
    dfs(0, -1, true, false, &digits, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::count_without_adjacent_equals;

    #[test]
    fn digit_dp_counts_numbers_without_equal_adjacent_digits() {
        assert_eq!(count_without_adjacent_equals(25), 24);
    }
}
