//! Ternary search on a unimodal function.
//!
//! Ternary search is the continuous-search analogue of binary search for unimodal objectives. If a
//! function strictly decreases and then increases, or vice versa, we can compare two interior
//! points and discard one third of the interval each iteration.
//!
//! # Example
//!
//! ```
//! use project1::advanced::searching_topics::ternary_search::ternary_search;
//!
//! let answer = ternary_search(-10.0, 10.0, |x| (x - 2.0) * (x - 2.0));
//! assert!((answer - 2.0).abs() < 1e-4);
//! ```

pub fn ternary_search(mut left: f64, mut right: f64, f: impl Fn(f64) -> f64) -> f64 {
    for _ in 0..200 {
        let first = left + (right - left) / 3.0;
        let second = right - (right - left) / 3.0;
        if f(first) <= f(second) {
            right = second;
        } else {
            left = first;
        }
    }
    (left + right) / 2.0
}

#[cfg(test)]
mod tests {
    use super::ternary_search;

    #[test]
    fn ternary_search_finds_the_minimum_of_a_unimodal_function() {
        let answer = ternary_search(-10.0, 10.0, |x| (x - 2.0) * (x - 2.0));
        assert!((answer - 2.0).abs() < 1e-4);
    }
}
