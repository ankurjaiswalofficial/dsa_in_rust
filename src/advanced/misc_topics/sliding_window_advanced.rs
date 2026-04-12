//! Advanced sliding window patterns.
//!
//! Sliding windows are not only for fixed-size windows. Many problems maintain a variable-size
//! window under a feasibility condition and shrink it whenever that condition breaks.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::sliding_window_advanced::longest_substring_with_at_most_k_distinct;
//!
//! assert_eq!(longest_substring_with_at_most_k_distinct("eceba", 2), 3);
//! ```

use std::collections::HashMap;

pub fn longest_substring_with_at_most_k_distinct(text: &str, k: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let mut count = HashMap::<char, usize>::new();
    let (mut left, mut answer) = (0usize, 0usize);
    for right in 0..chars.len() {
        *count.entry(chars[right]).or_insert(0) += 1;
        while count.len() > k {
            let entry = count
                .get_mut(&chars[left])
                .expect("left character should be in the window");
            *entry -= 1;
            if *entry == 0 {
                count.remove(&chars[left]);
            }
            left += 1;
        }
        answer = answer.max(right - left + 1);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::longest_substring_with_at_most_k_distinct;

    #[test]
    fn advanced_sliding_window_tracks_k_distinct_characters() {
        assert_eq!(longest_substring_with_at_most_k_distinct("eceba", 2), 3);
    }
}
