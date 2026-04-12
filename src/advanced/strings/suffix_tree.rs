//! Compressed suffix tree style construction.
//!
//! Full Ukkonen suffix trees are linear-time but large and subtle. This implementation moves a step
//! beyond a naive suffix trie by compressing chains of single-child edges into labeled substrings.
//! That keeps substring queries fast enough for learning while preserving the core "all suffixes in
//! one indexed structure" idea.
//!
//! # Example
//!
//! ```
//! use project1::advanced::strings::suffix_tree::CompressedSuffixTree;
//!
//! let tree = CompressedSuffixTree::new("banana");
//! assert!(tree.contains("ana"));
//! assert!(!tree.contains("apple"));
//! ```

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct TrieNode {
    next: HashMap<u8, usize>,
}

#[derive(Debug, Clone)]
struct CompressedEdge {
    label: Vec<u8>,
    to: usize,
}

#[derive(Debug, Clone, Default)]
struct CompressedNode {
    edges: Vec<CompressedEdge>,
}

#[derive(Debug, Clone, Default)]
pub struct CompressedSuffixTree {
    nodes: Vec<CompressedNode>,
}

impl CompressedSuffixTree {
    pub fn new(text: &str) -> Self {
        let trie = build_suffix_trie(text.as_bytes());
        compress_trie(&trie, 0)
    }

    pub fn contains(&self, pattern: &str) -> bool {
        let bytes = pattern.as_bytes();
        let mut node = 0usize;
        let mut index = 0usize;
        while index < bytes.len() {
            let Some(edge) = self.nodes[node]
                .edges
                .iter()
                .find(|edge| edge.label.first().copied() == Some(bytes[index]))
            else {
                return false;
            };
            let mut matched = 0usize;
            while matched < edge.label.len() && index + matched < bytes.len() {
                if edge.label[matched] != bytes[index + matched] {
                    return false;
                }
                matched += 1;
            }
            index += matched;
            if matched < edge.label.len() {
                return index == bytes.len();
            }
            node = edge.to;
        }
        true
    }
}

fn build_suffix_trie(text: &[u8]) -> Vec<TrieNode> {
    let mut trie = vec![TrieNode::default()];
    for start in 0..text.len() {
        let mut node = 0usize;
        for &byte in &text[start..] {
            let next = if let Some(&child) = trie[node].next.get(&byte) {
                child
            } else {
                trie.push(TrieNode::default());
                let child = trie.len() - 1;
                trie[node].next.insert(byte, child);
                child
            };
            node = next;
        }
    }
    trie
}

fn compress_trie(trie: &[TrieNode], root: usize) -> CompressedSuffixTree {
    let mut tree = CompressedSuffixTree {
        nodes: vec![CompressedNode::default()],
    };

    fn build(node: usize, trie: &[TrieNode], tree: &mut CompressedSuffixTree) -> usize {
        let current = tree.nodes.len() - 1;
        let children: Vec<(u8, usize)> = trie[node].next.iter().map(|(&byte, &next)| (byte, next)).collect();
        for (byte, next) in children {
            let mut label = vec![byte];
            let mut cursor = next;
            while trie[cursor].next.len() == 1 {
                let (&next_byte, &next_node) = trie[cursor].next.iter().next().expect("single child exists");
                label.push(next_byte);
                cursor = next_node;
            }
            tree.nodes.push(CompressedNode::default());
            let child_index = tree.nodes.len() - 1;
            build_with_target(cursor, trie, tree, child_index);
            tree.nodes[current].edges.push(CompressedEdge { label, to: child_index });
        }
        current
    }

    fn build_with_target(node: usize, trie: &[TrieNode], tree: &mut CompressedSuffixTree, target: usize) {
        let children: Vec<(u8, usize)> = trie[node].next.iter().map(|(&byte, &next)| (byte, next)).collect();
        for (byte, next) in children {
            let mut label = vec![byte];
            let mut cursor = next;
            while trie[cursor].next.len() == 1 {
                let (&next_byte, &next_node) = trie[cursor].next.iter().next().expect("single child exists");
                label.push(next_byte);
                cursor = next_node;
            }
            tree.nodes.push(CompressedNode::default());
            let child_index = tree.nodes.len() - 1;
            build_with_target(cursor, trie, tree, child_index);
            tree.nodes[target].edges.push(CompressedEdge { label, to: child_index });
        }
    }

    build(root, trie, &mut tree);
    tree
}

#[cfg(test)]
mod tests {
    use super::CompressedSuffixTree;

    #[test]
    fn compressed_suffix_tree_answers_substring_queries() {
        let tree = CompressedSuffixTree::new("banana");
        assert!(tree.contains("ana"));
        assert!(tree.contains("nana"));
        assert!(tree.contains("ban"));
        assert!(!tree.contains("apple"));
    }
}
