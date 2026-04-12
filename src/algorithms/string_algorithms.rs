//! String algorithms.
//!
//! Strings are rich enough that entire algorithm families exist for matching, prefix analysis,
//! hashing, and suffix processing.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::string_algorithms::{kmp_search, z_algorithm};
//!
//! assert_eq!(kmp_search("ababcabcabababd", "ababd"), vec![10]);
//! assert_eq!(z_algorithm("aaaaa"), vec![0, 4, 3, 2, 1]);
//! ```

use std::collections::HashMap;

pub fn string_traversal(text: &str) -> Vec<char> {
    text.chars().collect()
}

pub fn is_palindrome(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

pub fn are_anagrams(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut counts = HashMap::new();
    for ch in left.chars() {
        *counts.entry(ch).or_insert(0_i32) += 1;
    }
    for ch in right.chars() {
        let Some(count) = counts.get_mut(&ch) else {
            return false;
        };
        *count -= 1;
        if *count == 0 {
            counts.remove(&ch);
        }
    }
    counts.is_empty()
}

pub fn naive_pattern_match(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let mut answer = Vec::new();
    if pattern_bytes.len() > text_bytes.len() {
        return answer;
    }
    for start in 0..=text_bytes.len() - pattern_bytes.len() {
        if &text_bytes[start..start + pattern_bytes.len()] == pattern_bytes {
            answer.push(start);
        }
    }
    answer
}

pub fn prefix_function(pattern: &str) -> Vec<usize> {
    let bytes = pattern.as_bytes();
    let mut pi = vec![0; bytes.len()];
    for i in 1..bytes.len() {
        let mut j = pi[i - 1];
        while j > 0 && bytes[i] != bytes[j] {
            j = pi[j - 1];
        }
        if bytes[i] == bytes[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let pi = prefix_function(pattern);
    let mut result = Vec::new();
    let mut j = 0;

    for (i, &byte) in text_bytes.iter().enumerate() {
        while j > 0 && byte != pattern_bytes[j] {
            j = pi[j - 1];
        }
        if byte == pattern_bytes[j] {
            j += 1;
        }
        if j == pattern_bytes.len() {
            result.push(i + 1 - j);
            j = pi[j - 1];
        }
    }
    result
}

pub fn z_algorithm(text: &str) -> Vec<usize> {
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut z = vec![0; n];
    let (mut left, mut right) = (0, 0);
    for i in 1..n {
        if i <= right {
            z[i] = (right - i + 1).min(z[i - left]);
        }
        while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > right + 1 {
            left = i;
            right = i + z[i] - 1;
        }
    }
    z
}

pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || pattern.len() > text.len() {
        return Vec::new();
    }
    const BASE: u64 = 911_382_323;
    const MOD: u64 = 1_000_000_007;
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let mut power = 1;
    let mut pattern_hash = 0;
    let mut window_hash = 0;
    for i in 0..pattern.len() {
        pattern_hash = (pattern_hash * BASE + pattern_bytes[i] as u64) % MOD;
        window_hash = (window_hash * BASE + text_bytes[i] as u64) % MOD;
        if i + 1 < pattern.len() {
            power = (power * BASE) % MOD;
        }
    }

    let mut matches = Vec::new();
    for i in 0..=text.len() - pattern.len() {
        if window_hash == pattern_hash && &text[i..i + pattern.len()] == pattern {
            matches.push(i);
        }
        if i + pattern.len() < text.len() {
            let outgoing = (text_bytes[i] as u64 * power) % MOD;
            window_hash = (MOD + window_hash + MOD - outgoing) % MOD;
            window_hash = (window_hash * BASE + text_bytes[i + pattern.len()] as u64) % MOD;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::{
        are_anagrams, is_palindrome, kmp_search, naive_pattern_match, prefix_function, rabin_karp,
        string_traversal, z_algorithm,
    };

    #[test]
    fn string_algorithms_find_patterns() {
        assert_eq!(string_traversal("abc"), vec!['a', 'b', 'c']);
        assert!(is_palindrome("level"));
        assert!(are_anagrams("listen", "silent"));
        assert_eq!(naive_pattern_match("abracadabra", "abra"), vec![0, 7]);
        assert_eq!(prefix_function("ababc"), vec![0, 0, 1, 2, 0]);
        assert_eq!(kmp_search("aaaaa", "aa"), vec![0, 1, 2, 3]);
        assert_eq!(rabin_karp("abracadabra", "abra"), vec![0, 7]);
    }

    #[test]
    fn z_algorithm_builds_prefix_match_lengths() {
        assert_eq!(z_algorithm("aaaaa"), vec![0, 4, 3, 2, 1]);
        assert_eq!(z_algorithm("abacaba"), vec![0, 0, 1, 0, 3, 0, 1]);
    }
}
