//! Bitset optimization.
//!
//! Bitsets pack many boolean states into machine words so bulk set operations run much faster than
//! scalar loops.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::bitset_optimization::subset_sum_possible;
//!
//! assert!(subset_sum_possible(&[3, 5, 7], 10));
//! assert!(!subset_sum_possible(&[3, 5, 7], 6));
//! ```

pub fn subset_sum_possible(values: &[usize], target: usize) -> bool {
    let mut bitset = vec![false; target + 1];
    bitset[0] = true;
    for &value in values {
        for sum in (value..=target).rev() {
            bitset[sum] |= bitset[sum - value];
        }
    }
    bitset[target]
}

#[cfg(test)]
mod tests {
    use super::subset_sum_possible;

    #[test]
    fn bitset_style_dp_solves_subset_sum() {
        assert!(subset_sum_possible(&[3, 5, 7], 10));
        assert!(!subset_sum_possible(&[3, 5, 7], 6));
    }
}

