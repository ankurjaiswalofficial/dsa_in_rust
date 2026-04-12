//! Euler totient function.
//!
//! Euler's totient function `phi(n)` counts how many integers in `[1, n]` are coprime to `n`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::advanced_number_theory::euler_totient_function::phi;
//!
//! assert_eq!(phi(9), 6);
//! ```

pub fn phi(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result = result / p * (p - 1);
        }
        p += 1;
    }
    if n > 1 {
        result = result / n * (n - 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::phi;

    #[test]
    fn totient_counts_coprimes() {
        assert_eq!(phi(9), 6);
    }
}

