//! LCP array with Kasai's algorithm.
//!
//! Once the suffix array is known, the LCP array stores the longest common prefix length between
//! adjacent suffixes in that order. Kasai's algorithm computes it in linear time.
//!
//! # Example
//!
//! ```
//! use project1::advanced::strings::lcp_array::lcp_array;
//!
//! assert_eq!(lcp_array("banana", &[5, 3, 1, 0, 4, 2]), vec![0, 1, 3, 0, 0, 2]);
//! ```

pub fn lcp_array(text: &str, suffix_array: &[usize]) -> Vec<usize> {
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut rank = vec![0; n];
    for (i, &suffix) in suffix_array.iter().enumerate() {
        rank[suffix] = i;
    }
    let mut lcp = vec![0; n];
    let mut common = 0;
    for i in 0..n {
        if rank[i] == 0 {
            continue;
        }
        let j = suffix_array[rank[i] - 1];
        while i + common < n && j + common < n && bytes[i + common] == bytes[j + common] {
            common += 1;
        }
        lcp[rank[i]] = common;
        common = common.saturating_sub(1);
    }
    lcp
}

#[cfg(test)]
mod tests {
    use super::lcp_array;

    #[test]
    fn lcp_array_matches_the_suffix_array_neighbors() {
        assert_eq!(lcp_array("banana", &[5, 3, 1, 0, 4, 2]), vec![0, 1, 3, 0, 0, 2]);
    }
}
