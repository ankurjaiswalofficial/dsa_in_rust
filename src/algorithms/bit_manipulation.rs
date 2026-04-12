//! Bit manipulation fundamentals.
//!
//! Bitwise reasoning is useful for compact state representation, subset DP,
//! parity tricks, and constant-factor optimizations.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::bit_manipulation::{is_power_of_two, set_bit};
//!
//! assert_eq!(set_bit(0b0010, 0), 0b0011);
//! assert!(is_power_of_two(16));
//! ```

pub fn set_bit(value: u32, bit: u32) -> u32 {
    value | (1 << bit)
}

pub fn unset_bit(value: u32, bit: u32) -> u32 {
    value & !(1 << bit)
}

pub fn toggle_bit(value: u32, bit: u32) -> u32 {
    value ^ (1 << bit)
}

pub fn is_power_of_two(value: u32) -> bool {
    value != 0 && (value & (value - 1)) == 0
}

pub fn xor_single_number(values: &[u32]) -> u32 {
    values.iter().fold(0, |acc, &value| acc ^ value)
}

pub fn enumerate_submasks(mask: u32) -> Vec<u32> {
    let mut submasks = Vec::new();
    let mut submask = mask;
    loop {
        submasks.push(submask);
        if submask == 0 {
            break;
        }
        submask = (submask - 1) & mask;
    }
    submasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_operations_work() {
        assert_eq!(set_bit(0b0010, 0), 0b0011);
        assert_eq!(unset_bit(0b0111, 1), 0b0101);
        assert_eq!(toggle_bit(0b0101, 0), 0b0100);
        assert!(is_power_of_two(16));
        assert!(!is_power_of_two(18));
    }

    #[test]
    fn xor_and_masking_examples_work() {
        assert_eq!(xor_single_number(&[2, 3, 2, 4, 4]), 3);
        assert_eq!(enumerate_submasks(0b101), vec![0b101, 0b100, 0b001, 0b000]);
    }
}
