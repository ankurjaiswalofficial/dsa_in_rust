//! Rolling hash.
//!
//! Rolling hash updates a window hash in constant time as the window moves. It is useful for
//! Rabin-Karp string matching and fast substring comparison with a small collision risk.
//!
//! # Example
//!
//! ```
//! use project1::advanced::strings::rolling_hash::find_occurrences;
//!
//! assert_eq!(find_occurrences("abracadabra", "abra"), vec![0, 7]);
//! ```

const BASE: u64 = 911_382_323;
const MOD: u64 = 1_000_000_007;

fn hash(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .fold(0, |acc, &byte| (acc * BASE + byte as u64 + 1) % MOD)
}

pub fn find_occurrences(text: &str, pattern: &str) -> Vec<usize> {
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    if pattern.is_empty() || pattern.len() > text.len() {
        return Vec::new();
    }
    let pattern_hash = hash(pattern);
    let mut power = 1;
    for _ in 1..pattern.len() {
        power = (power * BASE) % MOD;
    }
    let mut current = hash(&text[..pattern.len()]);
    let mut answer = Vec::new();
    for start in 0..=text.len() - pattern.len() {
        if current == pattern_hash && &text[start..start + pattern.len()] == pattern {
            answer.push(start);
        }
        if start + pattern.len() < text.len() {
            let outgoing = ((text[start] as u64 + 1) * power) % MOD;
            current = (MOD + current - outgoing) % MOD;
            current = (current * BASE + text[start + pattern.len()] as u64 + 1) % MOD;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::find_occurrences;

    #[test]
    fn rolling_hash_finds_pattern_occurrences() {
        assert_eq!(find_occurrences("abracadabra", "abra"), vec![0, 7]);
    }
}
