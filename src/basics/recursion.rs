//! Recursion examples and notes.
//!
//! Recursion solves a problem by reducing it to smaller versions of the same
//! problem. It is especially useful for trees, divide-and-conquer, and search.
//!
//! # Covered topics
//!
//! - Base case, call stack, integer overflow and stack overflow (video: DSA in Rust, episode 7)
//! - Recursion trees, memoization and fast power (video: DSA in Rust, episode 8)
//!
//! # Example
//!
//! ```
//! use project1::basics::recursion::{factorial, fibonacci};
//!
//! assert_eq!(factorial(5), 120);
//! assert_eq!(fibonacci(7), 13);
//! ```

pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

pub fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

/// Counts down from `n` in steps of two; returns how many steps it took.
///
/// The base case is `n <= 0`, not `n == 0`: an odd `n` jumps from 1 to -1, and
/// with `n == 0` the base case is never reached ("thread 'main' has overflowed
/// its stack").
pub fn countdown_by_two(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    1 + countdown_by_two(n - 2)
}

/// Recursive sum of a slice: the first item plus the sum of the rest.
/// O(n) time and O(n) stack: a million items overflow the default stack.
pub fn sum_recursive(v: &[i64]) -> i64 {
    if v.is_empty() {
        return 0;
    }
    v[0] + sum_recursive(&v[1..])
}

/// The same sum as a loop: O(n) time, O(1) stack.
pub fn sum_iterative(v: &[i64]) -> i64 {
    let mut total = 0;
    for &x in v {
        total += x;
    }
    total
}

/// Naive recursive Fibonacci that counts its calls: fib(30) makes 2,692,537.
pub fn fib_naive_counted(n: u64, calls: &mut u64) -> u64 {
    *calls += 1;
    if n < 2 {
        return n;
    }
    fib_naive_counted(n - 1, calls) + fib_naive_counted(n - 2, calls)
}

/// Memoized Fibonacci: each `n` is solved once, so fib(30) makes 59 calls.
/// `memo` needs `n + 1` slots, all `None` at the start. Forgetting the
/// `memo[n] = Some(v)` line makes it as slow as the naive version.
pub fn fib_memo(n: usize, memo: &mut Vec<Option<u64>>, calls: &mut u64) -> u64 {
    *calls += 1;
    if let Some(v) = memo[n] {
        return v;
    }
    let v = if n < 2 { n as u64 } else { fib_memo(n - 1, memo, calls) + fib_memo(n - 2, memo, calls) };
    memo[n] = Some(v);
    v
}

/// Fast power: `x^n mod m` from `x^(n/2)`, one recursive call per halving,
/// so O(log n) calls (41 calls for n = 10^12). Needs `m <= 2^32` so that
/// `half * half` fits in a u64.
pub fn power_mod(x: u64, n: u64, m: u64, calls: &mut u64) -> u64 {
    *calls += 1;
    if n == 0 {
        return 1 % m;
    }
    let half = power_mod(x, n / 2, m, calls);
    if n % 2 == 0 {
        half * half % m
    } else {
        half * half % m * (x % m) % m
    }
}

pub fn recursion_summary() -> &'static str {
    "Recursive code needs a base case, progress toward it, and controlled repeated work."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_handles_small_values() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn fibonacci_handles_base_and_general_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn countdown_reaches_the_base_case_for_odd_numbers() {
        assert_eq!(countdown_by_two(5), 3);
        assert_eq!(countdown_by_two(4), 2);
        assert_eq!(countdown_by_two(0), 0);
    }

    #[test]
    fn recursive_and_iterative_sums_agree() {
        let v = [3, 1, 4, 1, 5];
        assert_eq!(sum_recursive(&v), 14);
        assert_eq!(sum_iterative(&v), 14);
        assert_eq!(sum_recursive(&[]), 0);
        let big: Vec<i64> = (1..=1_000_000).collect();
        assert_eq!(sum_iterative(&big), 500_000_500_000); // the loop needs no deep stack
    }

    #[test]
    fn factorial_20_is_the_largest_that_fits_u64() {
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
        assert!(21u64.checked_mul(factorial(20)).is_none()); // 21! overflows
    }

    #[test]
    fn memo_makes_fibonacci_linear() {
        let mut calls = 0;
        assert_eq!(fib_naive_counted(30, &mut calls), 832_040);
        assert_eq!(calls, 2_692_537);
        let mut calls = 0;
        assert_eq!(fib_memo(30, &mut vec![None; 31], &mut calls), 832_040);
        assert_eq!(calls, 59);
        let mut calls = 0;
        assert_eq!(fib_memo(90, &mut vec![None; 91], &mut calls), 2_880_067_194_370_816_120);
    }

    #[test]
    fn fast_power_uses_logarithmic_calls() {
        const M: u64 = 1_000_000_007;
        for (n, want, want_calls) in [(10, 1024, 5), (1_000, 688_423_210, 11), (1_000_000, 235_042_059, 21), (1_000_000_000_000, 959_366_170, 41)] {
            let mut calls = 0;
            assert_eq!(power_mod(2, n, M, &mut calls), want);
            assert_eq!(calls, want_calls);
        }
    }
}
