//! Manacher's algorithm.
//!
//! Manacher's algorithm computes palindrome radii around every center in linear time. That makes it
//! the standard optimal solution for longest palindromic substring.
//!
//! # Example
//!
//! ```
//! use project1::advanced::strings::manachers_algorithm::longest_palindromic_substring;
//!
//! assert_eq!(longest_palindromic_substring("babad"), "bab");
//! ```

pub fn longest_palindromic_substring(text: &str) -> String {
    let transformed: Vec<char> = std::iter::once('^')
        .chain(text.chars().flat_map(|ch| ['#', ch]))
        .chain(['#', '$'])
        .collect();
    let mut radius = vec![0usize; transformed.len()];
    let (mut center, mut right) = (0usize, 0usize);
    let (mut best_center, mut best_radius) = (0usize, 0usize);
    for i in 1..transformed.len() - 1 {
        if i < right {
            let mirror = 2 * center - i;
            radius[i] = radius[mirror].min(right - i);
        }
        while transformed[i + radius[i] + 1] == transformed[i - radius[i] - 1] {
            radius[i] += 1;
        }
        if i + radius[i] > right {
            center = i;
            right = i + radius[i];
        }
        if radius[i] > best_radius {
            best_radius = radius[i];
            best_center = i;
        }
    }
    let start = (best_center - best_radius) / 2;
    text.chars().skip(start).take(best_radius).collect()
}

#[cfg(test)]
mod tests {
    use super::longest_palindromic_substring;

    #[test]
    fn manachers_algorithm_finds_a_longest_palindrome() {
        let answer = longest_palindromic_substring("babad");
        assert!(answer == "bab" || answer == "aba");
    }
}
