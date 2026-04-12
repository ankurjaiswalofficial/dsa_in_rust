//! Trie basics.
//!
//! A trie stores strings character by character. Instead of comparing an entire word at once like
//! a balanced BST or hash table, it walks one character edge at a time. That makes prefix search
//! particularly natural and keeps insert/search complexity proportional to the word length.
//!
//! This advanced-basics wrapper reuses the core trie implementation from the data-structures
//! module so the same code stays consistent across the crate.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trie_topics::trie_basics::Trie;
//!
//! let mut trie = Trie::new();
//! trie.insert("cat");
//! trie.insert("car");
//! assert!(trie.contains("cat"));
//! assert!(trie.starts_with("ca"));
//! assert!(!trie.contains("cap"));
//! ```

pub use crate::data_structures::trie::Trie;

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn trie_basics_support_insert_search_and_prefix() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("app");
        assert!(trie.contains("app"));
        assert!(trie.contains("apple"));
        assert!(trie.starts_with("appl"));
        assert!(!trie.contains("apply"));
    }
}
