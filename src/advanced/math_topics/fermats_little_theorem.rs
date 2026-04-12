//! Fermat's little theorem.
//!
//! For a prime `p` and a number `a` not divisible by `p`, Fermat's little theorem states that
//! `a^(p-1) ≡ 1 (mod p)`. In programming contests and modular arithmetic code, that often gives a
//! fast way to compute inverses as `a^(p-2) mod p`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::fermats_little_theorem::mod_inverse_prime;
//!
//! assert_eq!(mod_inverse_prime(3, 11), 4);
//! ```

pub fn mod_pow(mut base: i64, mut exponent: i64, modulus: i64) -> i64 {
    let mut result = 1_i64;
    base = base.rem_euclid(modulus);
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base).rem_euclid(modulus);
        }
        base = (base * base).rem_euclid(modulus);
        exponent >>= 1;
    }
    result
}

pub fn mod_inverse_prime(value: i64, prime_modulus: i64) -> i64 {
    mod_pow(value, prime_modulus - 2, prime_modulus)
}

#[cfg(test)]
mod tests {
    use super::{mod_inverse_prime, mod_pow};

    #[test]
    fn modular_power_works() {
        assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
    }

    #[test]
    fn fermat_inverse_works_for_prime_modulus() {
        assert_eq!(mod_inverse_prime(3, 11), 4);
    }
}
