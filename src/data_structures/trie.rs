//! Trie for lowercase ASCII strings.
//!
//! A trie stores strings character by character, which makes prefix queries efficient.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::trie::Trie;
//!
//! let mut trie = Trie::new();
//! trie.insert("cat");
//! trie.insert("car");
//! assert!(trie.contains("cat"));
//! assert!(trie.starts_with("ca"));
//! assert!(!trie.contains("cap"));
//! ```

#[derive(Debug, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    terminal: bool,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            terminal: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for byte in word.bytes() {
            let index = (byte - b'a') as usize;
            node = node.children[index]
                .get_or_insert_with(|| Box::new(TrieNode::default()))
                .as_mut();
        }
        node.terminal = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        self.find_node(word).is_some_and(|node| node.terminal)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, text: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for byte in text.bytes() {
            let index = (byte - b'a') as usize;
            node = node.children[index].as_deref()?;
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn trie_handles_words_and_prefixes() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("app");
        assert!(trie.contains("app"));
        assert!(trie.contains("apple"));
        assert!(trie.starts_with("appl"));
        assert!(!trie.contains("apply"));
    }
}
