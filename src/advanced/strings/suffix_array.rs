//! Suffix array construction.
//!
//! A suffix array stores the starting positions of all suffixes in lexicographic order. It is much
//! lighter than a suffix tree and is often paired with an LCP array for substring queries.
//!
//! This implementation uses the simple `O(n log^2 n)` doubling method, which is ideal for
//! educational code.
//!
//! # Example
//!
//! ```
//! use project1::advanced::strings::suffix_array::suffix_array;
//!
//! assert_eq!(suffix_array("banana"), vec![5, 3, 1, 0, 4, 2]);
//! ```

pub fn suffix_array(text: &str) -> Vec<usize> {
    let bytes = text.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return Vec::new();
    }
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i32> = bytes.iter().map(|&b| b as i32).collect();
    let mut temp = vec![0; n];
    let mut gap = 1;
    while gap < n {
        sa.sort_by_key(|&i| (rank[i], rank.get(i + gap).copied().unwrap_or(-1)));
        temp[0] = 0;
        for i in 1..n {
            let previous = sa[i - 1];
            let current = sa[i];
            temp[i] = temp[i - 1]
                + i32::from(
                    (rank[previous], rank.get(previous + gap).copied().unwrap_or(-1))
                        != (rank[current], rank.get(current + gap).copied().unwrap_or(-1)),
                );
        }
        for i in 0..n {
            rank[sa[i]] = temp[i];
        }
        if rank[sa[n - 1]] == (n - 1) as i32 {
            break;
        }
        gap <<= 1;
    }
    sa
}

#[cfg(test)]
mod tests {
    use super::suffix_array;

    #[test]
    fn suffix_array_orders_suffixes_lexicographically() {
        assert_eq!(suffix_array("banana"), vec![5, 3, 1, 0, 4, 2]);
    }
}
