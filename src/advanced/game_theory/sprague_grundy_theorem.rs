//! Sprague-Grundy theorem.
//!
//! The Sprague-Grundy theorem says every impartial game under normal play is equivalent to a Nim
//! heap of some size, its Grundy number. The xor of independent subgames decides the winner.
//!
//! # Example
//!
//! ```
//! use project1::advanced::game_theory::sprague_grundy_theorem::disjunctive_sum_is_winning;
//!
//! assert!(disjunctive_sum_is_winning(&[1, 2]));
//! assert!(!disjunctive_sum_is_winning(&[3, 3]));
//! ```

pub fn disjunctive_sum_is_winning(grundy_values: &[usize]) -> bool {
    grundy_values.iter().fold(0, |acc, &value| acc ^ value) != 0
}

#[cfg(test)]
mod tests {
    use super::disjunctive_sum_is_winning;

    #[test]
    fn sprague_grundy_uses_xor_of_subgames() {
        assert!(disjunctive_sum_is_winning(&[1, 2]));
        assert!(!disjunctive_sum_is_winning(&[3, 3]));
    }
}

