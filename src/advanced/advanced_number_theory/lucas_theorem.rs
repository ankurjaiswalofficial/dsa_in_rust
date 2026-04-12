//! Lucas theorem.
//!
//! Lucas theorem reduces `nCr mod p` for prime `p` into digit-wise combinations in base `p`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::advanced_number_theory::lucas_theorem::ncr_mod_prime_lucas;
//!
//! assert_eq!(ncr_mod_prime_lucas(5, 2, 7), 3);
//! ```

pub fn ncr_mod_prime_lucas(mut n: u64, mut r: u64, p: u64) -> u64 {
    let mut result = 1;
    while n > 0 || r > 0 {
        let ni = n % p;
        let ri = r % p;
        if ri > ni {
            return 0;
        }
        result = result * small_ncr_mod_prime(ni, ri, p) % p;
        n /= p;
        r /= p;
    }
    result
}

fn small_ncr_mod_prime(n: u64, r: u64, p: u64) -> u64 {
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..r {
        numerator = numerator * (n - i) % p;
        denominator = denominator * (i + 1) % p;
    }
    numerator * mod_pow(denominator, p - 2, p) % p
}

fn mod_pow(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modu;
        }
        base = base * base % modu;
        exp >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::ncr_mod_prime_lucas;

    #[test]
    fn lucas_theorem_computes_ncr_mod_prime() {
        assert_eq!(ncr_mod_prime_lucas(5, 2, 7), 3);
    }
}
