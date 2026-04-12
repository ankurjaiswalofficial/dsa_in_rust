//! Greedy proof techniques.
//!
//! Greedy algorithms are not complete without proof. The most common proof styles are:
//! - Exchange argument
//! - Staying-ahead proof
//! - Structural induction on the greedy choice
//!
//! # Example
//!
//! ```
//! use project1::advanced::algorithm_design::greedy_proof_techniques::proof_styles;
//!
//! assert!(proof_styles().contains(&"Exchange argument"));
//! ```

pub fn proof_styles() -> &'static [&'static str] {
    &["Exchange argument", "Staying ahead", "Greedy-choice induction"]
}

#[cfg(test)]
mod tests {
    use super::proof_styles;

    #[test]
    fn greedy_proof_styles_are_listed() {
        assert!(proof_styles().contains(&"Exchange argument"));
    }
}

