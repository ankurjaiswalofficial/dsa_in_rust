//! Recursion examples and notes.
//!
//! Recursion solves a problem by reducing it to smaller versions of the same
//! problem. It is especially useful for trees, divide-and-conquer, and search.
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
}
