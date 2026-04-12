//! Modular inverse.
//!
//! A modular inverse of `a` modulo `m` is a number `x` such that `a * x ≡ 1 (mod m)`. It exists
//! exactly when `a` and `m` are coprime.
//!
//! This implementation uses the extended Euclidean algorithm.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::modular_inverse::mod_inverse;
//!
//! assert_eq!(mod_inverse(3, 11), Some(4));
//! assert_eq!(mod_inverse(6, 9), None);
//! ```

use crate::advanced::math_topics::extended_euclidean_algorithm::extended_gcd;

pub fn mod_inverse(a: i64, modulus: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, modulus);
    if g != 1 {
        None
    } else {
        Some(x.rem_euclid(modulus))
    }
}

#[cfg(test)]
mod tests {
    use super::mod_inverse;

    #[test]
    fn modular_inverse_finds_the_inverse_when_it_exists() {
        assert_eq!(mod_inverse(3, 11), Some(4));
    }

    #[test]
    fn modular_inverse_returns_none_when_gcd_is_not_one() {
        assert_eq!(mod_inverse(6, 9), None);
    }
}
