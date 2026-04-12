//! Chinese remainder theorem.
//!
//! CRT combines congruences like `x ≡ a_i (mod m_i)` into one solution modulo the product of the
//! moduli, assuming the moduli are pairwise coprime.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::chinese_remainder_theorem::crt;
//!
//! assert_eq!(crt(&[(2, 3), (3, 5), (2, 7)]), Some((23, 105)));
//! ```

use crate::advanced::math_topics::modular_inverse::mod_inverse;

pub fn crt(congruences: &[(i64, i64)]) -> Option<(i64, i64)> {
    let modulus = congruences.iter().map(|&(_, m)| m).product::<i64>();
    let mut answer = 0;
    for &(remainder, current_modulus) in congruences {
        let partial = modulus / current_modulus;
        let inverse = mod_inverse(partial.rem_euclid(current_modulus), current_modulus)?;
        answer += remainder * partial * inverse;
    }
    Some((answer.rem_euclid(modulus), modulus))
}

#[cfg(test)]
mod tests {
    use super::crt;

    #[test]
    fn crt_combines_pairwise_coprime_congruences() {
        assert_eq!(crt(&[(2, 3), (3, 5), (2, 7)]), Some((23, 105)));
    }
}
