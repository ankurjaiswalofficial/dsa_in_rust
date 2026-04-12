//! Extended Euclidean algorithm.
//!
//! The extended Euclidean algorithm computes not only `gcd(a, b)`, but also coefficients `x` and
//! `y` such that `ax + by = gcd(a, b)`. Those coefficients are the key ingredient behind modular
//! inverses and the Chinese remainder theorem.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::extended_euclidean_algorithm::extended_gcd;
//!
//! let (g, x, y) = extended_gcd(30, 18);
//! assert_eq!(g, 6);
//! assert_eq!(30 * x + 18 * y, g);
//! ```

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a.abs(), a.signum(), 0);
    }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

#[cfg(test)]
mod tests {
    use super::extended_gcd;

    #[test]
    fn extended_gcd_returns_bezout_coefficients() {
        let (g, x, y) = extended_gcd(30, 18);
        assert_eq!(g, 6);
        assert_eq!(30 * x + 18 * y, g);
    }
}
