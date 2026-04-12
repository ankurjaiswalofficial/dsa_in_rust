//! Gray code.
//!
//! Consecutive Gray-code values differ by exactly one bit. That makes them useful in hardware,
//! state transitions, and some combinational-generation problems.
//!
//! # Example
//!
//! ```
//! use project1::advanced::bit_manipulation_topics::gray_code::gray_code_sequence;
//!
//! assert_eq!(gray_code_sequence(2), vec![0, 1, 3, 2]);
//! ```

pub fn gray(value: usize) -> usize {
    value ^ (value >> 1)
}

pub fn gray_code_sequence(bits: usize) -> Vec<usize> {
    (0..(1usize << bits)).map(gray).collect()
}

#[cfg(test)]
mod tests {
    use super::{gray, gray_code_sequence};

    #[test]
    fn gray_code_formula_works() {
        assert_eq!(gray(0), 0);
        assert_eq!(gray(1), 1);
        assert_eq!(gray(2), 3);
        assert_eq!(gray(3), 2);
    }

    #[test]
    fn gray_code_sequence_is_generated_correctly() {
        assert_eq!(gray_code_sequence(2), vec![0, 1, 3, 2]);
    }
}
