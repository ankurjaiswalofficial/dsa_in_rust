//! Polynomial multiplication.
//!
//! This file exposes polynomial multiplication as the core operation, independent of whether the
//! engine is naive, FFT-based, or NTT-based.
//!
//! # Example
//!
//! ```
//! use project1::advanced::algebra_polynomial::polynomial_multiplication::multiply;
//!
//! assert_eq!(multiply(&[2, 0, 1], &[1, 3]), vec![2, 6, 1, 3]);
//! ```

pub fn multiply(left: &[i64], right: &[i64]) -> Vec<i64> {
    let mut result = vec![0; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            result[i + j] += a * b;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn polynomial_multiplication_works() {
        assert_eq!(multiply(&[2, 0, 1], &[1, 3]), vec![2, 6, 1, 3]);
    }
}
