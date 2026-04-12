//! Prefix matching with tries.
//!
//! Prefix matching is one of the main reasons to use a trie. Once the prefix path exists, every
//! word below that point shares the same prefix. In practice, that idea powers autocomplete,
//! dictionary lookup, and prefix counting.
//!
//! The helper functions in this file keep the code lightweight:
//! - `count_matching_prefixes` checks whether the prefix exists in the trie.
//! - `filter_by_prefix` returns the concrete words that match.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trie_topics::prefix_matching::{count_matching_prefixes, filter_by_prefix};
//!
//! let words = ["apple", "app", "banana", "apply"];
//! assert_eq!(count_matching_prefixes(&words, "app"), 1);
//! assert_eq!(filter_by_prefix(&words, "app"), vec!["apple", "app", "apply"]);
//! ```

use crate::data_structures::trie::Trie;

pub fn count_matching_prefixes(words: &[&str], prefix: &str) -> usize {
    let mut trie = Trie::new();
    for &word in words {
        trie.insert(word);
    }
    usize::from(trie.starts_with(prefix))
}

pub fn filter_by_prefix<'a>(words: &'a [&'a str], prefix: &str) -> Vec<&'a str> {
    words
        .iter()
        .copied()
        .filter(|word| word.starts_with(prefix))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{count_matching_prefixes, filter_by_prefix};

    #[test]
    fn prefix_matching_finds_existing_prefixes() {
        let words = ["apple", "app", "banana", "apply"];
        assert_eq!(count_matching_prefixes(&words, "app"), 1);
        assert_eq!(count_matching_prefixes(&words, "cat"), 0);
    }

    #[test]
    fn prefix_matching_filters_the_words() {
        let words = ["apple", "app", "banana", "apply"];
        assert_eq!(filter_by_prefix(&words, "app"), vec!["apple", "app", "apply"]);
    }
}
