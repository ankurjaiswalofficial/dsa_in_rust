//! Math and number-theory helpers.
//!
//! Number theory appears constantly in combinatorics, hashing, optimized DP, and graph problems.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::math::{gcd, mod_pow, sieve_primes};
//!
//! assert_eq!(gcd(24, 18), 6);
//! assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
//! assert_eq!(sieve_primes(10), vec![2, 3, 5, 7]);
//! ```

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut divisor = 3;
    while divisor * divisor <= n {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

pub fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    base %= modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exponent >>= 1;
    }
    result
}

pub fn sieve_primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(value, &prime)| prime.then_some(value))
        .collect()
}

pub fn mod_add(a: u64, b: u64, modulus: u64) -> u64 {
    (a % modulus + b % modulus) % modulus
}

pub fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    (a % modulus * b % modulus) % modulus
}

#[cfg(test)]
mod tests {
    use super::{gcd, is_prime, lcm, mod_add, mod_mul, mod_pow, sieve_primes};

    #[test]
    fn math_helpers_cover_common_utilities() {
        assert!(is_prime(97));
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(mod_pow(3, 5, 13), 9);
        assert_eq!(mod_add(10, 8, 7), 4);
        assert_eq!(mod_mul(10, 8, 7), 3);
        assert_eq!(sieve_primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
