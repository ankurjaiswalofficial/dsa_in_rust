//! Subset generation using bits.
//!
//! When a collection has `n` elements, there are `2^n` subsets. A bitmask from `0` to `2^n - 1`
//! tells us exactly which elements are present.
//!
//! # Example
//!
//! ```
//! use project1::advanced::bit_manipulation_topics::subset_generation_using_bits::generate_subsets;
//!
//! let subsets = generate_subsets(&[1, 2]);
//! assert_eq!(subsets, vec![vec![], vec![1], vec![2], vec![1, 2]]);
//! ```

pub fn generate_subsets(values: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for mask in 0..(1usize << values.len()) {
        let mut subset = Vec::new();
        for (index, &value) in values.iter().enumerate() {
            if (mask >> index) & 1 == 1 {
                subset.push(value);
            }
        }
        result.push(subset);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::generate_subsets;

    #[test]
    fn subset_generation_enumerates_every_subset() {
        let subsets = generate_subsets(&[1, 2]);
        assert_eq!(subsets, vec![vec![], vec![1], vec![2], vec![1, 2]]);
    }
}
