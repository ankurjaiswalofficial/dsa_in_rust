//! Trie with insert, search, prefix check, and delete.
//!
//! The basic trie operations are:
//! - Insert: create missing nodes along the word path.
//! - Search: follow the path and check whether the last node is terminal.
//! - Delete: clear the terminal flag and prune nodes that are no longer needed.
//!
//! This file keeps the implementation explicit so the delete logic is easy to study.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trie_topics::insert_search_delete::EditableTrie;
//!
//! let mut trie = EditableTrie::default();
//! trie.insert("code");
//! trie.insert("coder");
//! assert!(trie.contains("code"));
//! assert!(trie.delete("code"));
//! assert!(!trie.contains("code"));
//! assert!(trie.contains("coder"));
//! ```

#[derive(Debug, Clone)]
struct Node {
    children: [Option<Box<Node>>; 26],
    terminal: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            terminal: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EditableTrie {
    root: Node,
}

impl EditableTrie {
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for byte in word.bytes() {
            let index = (byte - b'a') as usize;
            node = node.children[index]
                .get_or_insert_with(|| Box::new(Node::default()))
                .as_mut();
        }
        node.terminal = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        self.find(word).is_some_and(|node| node.terminal)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find(prefix).is_some()
    }

    pub fn delete(&mut self, word: &str) -> bool {
        delete(&mut self.root, word.as_bytes(), 0).0
    }

    fn find(&self, text: &str) -> Option<&Node> {
        let mut node = &self.root;
        for byte in text.bytes() {
            let index = (byte - b'a') as usize;
            node = node.children[index].as_deref()?;
        }
        Some(node)
    }
}

fn delete(node: &mut Node, bytes: &[u8], index: usize) -> (bool, bool) {
    if index == bytes.len() {
        if !node.terminal {
            return (false, false);
        }
        node.terminal = false;
        return (true, node.children.iter().all(Option::is_none));
    }

    let child_index = (bytes[index] - b'a') as usize;
    let Some(child) = node.children[child_index].as_mut() else {
        return (false, false);
    };

    let (deleted, prune_child) = delete(child, bytes, index + 1);
    if prune_child {
        node.children[child_index] = None;
    }

    let should_prune = !node.terminal && node.children.iter().all(Option::is_none);
    (deleted, should_prune)
}

#[cfg(test)]
mod tests {
    use super::EditableTrie;

    #[test]
    fn editable_trie_supports_insert_and_search() {
        let mut trie = EditableTrie::default();
        trie.insert("cat");
        trie.insert("car");
        assert!(trie.contains("cat"));
        assert!(trie.contains("car"));
        assert!(trie.starts_with("ca"));
    }

    #[test]
    fn editable_trie_supports_delete_without_breaking_shared_prefixes() {
        let mut trie = EditableTrie::default();
        trie.insert("code");
        trie.insert("coder");
        assert!(trie.delete("code"));
        assert!(!trie.contains("code"));
        assert!(trie.contains("coder"));
        assert!(!trie.delete("coding"));
    }
}
