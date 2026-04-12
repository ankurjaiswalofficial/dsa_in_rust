//! Grundy numbers.
//!
//! The Grundy number of a state is the mex of the Grundy numbers of all reachable states. A state
//! is losing exactly when its Grundy number is zero.
//!
//! # Example
//!
//! ```
//! use project1::advanced::game_theory::grundy_numbers::grundy_take_1_or_2;
//!
//! assert_eq!(grundy_take_1_or_2(0), 0);
//! assert_eq!(grundy_take_1_or_2(3), 0);
//! ```

pub fn grundy_take_1_or_2(n: usize) -> usize {
    match n % 3 {
        0 => 0,
        1 => 1,
        _ => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::grundy_take_1_or_2;

    #[test]
    fn grundy_numbers_match_the_take_1_or_2_pattern() {
        assert_eq!(grundy_take_1_or_2(0), 0);
        assert_eq!(grundy_take_1_or_2(3), 0);
    }
}

