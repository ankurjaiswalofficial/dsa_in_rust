//! Coordinate compression.
//!
//! Coordinate compression maps large or sparse values onto a small rank space while preserving
//! order. It is common before Fenwick trees, segment trees, or offline range processing.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::coordinate_compression::compress;
//!
//! assert_eq!(compress(&[100, 500, 100, -2]), vec![1, 2, 1, 0]);
//! ```

pub fn compress(values: &[i64]) -> Vec<usize> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    values
        .iter()
        .map(|value| sorted.binary_search(value).expect("value should exist after dedup"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::compress;

    #[test]
    fn coordinate_compression_maps_values_to_ranks() {
        assert_eq!(compress(&[100, 500, 100, -2]), vec![1, 2, 1, 0]);
    }
}
