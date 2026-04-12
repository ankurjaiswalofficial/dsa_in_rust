//! Mobius function.
//!
//! The Mobius function distinguishes square-free integers and is central to inversion formulas.
//!
//! # Example
//!
//! ```
//! use project1::advanced::advanced_number_theory::mobius_function::mobius_up_to;
//!
//! assert_eq!(mobius_up_to(5), vec![0, 1, -1, -1, 0, -1]);
//! ```

pub fn mobius_up_to(n: usize) -> Vec<i32> {
    let mut mu = vec![1; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();
    mu[0] = 0;
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            if i * p > n {
                break;
            }
            is_prime[i * p] = false;
            if i % p == 0 {
                mu[i * p] = 0;
                break;
            } else {
                mu[i * p] = -mu[i];
            }
        }
    }
    mu[1] = 1;
    mu
}

#[cfg(test)]
mod tests {
    use super::mobius_up_to;

    #[test]
    fn mobius_values_match_small_known_values() {
        assert_eq!(mobius_up_to(5), vec![0, 1, -1, -1, 0, -1]);
    }
}

