//! State compression.
//!
//! State compression encodes many small boolean or categorical choices into a compact integer
//! representation.
//!
//! # Example
//!
//! ```
//! use project1::advanced::problem_solving_patterns::state_compression::encode_bool_state;
//!
//! assert_eq!(encode_bool_state(&[true, false, true]), 5);
//! ```

pub fn encode_bool_state(flags: &[bool]) -> usize {
    let mut mask = 0usize;
    for (index, &flag) in flags.iter().enumerate() {
        if flag {
            mask |= 1 << index;
        }
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::encode_bool_state;

    #[test]
    fn state_compression_encodes_booleans_into_bits() {
        assert_eq!(encode_bool_state(&[true, false, true]), 5);
    }
}
