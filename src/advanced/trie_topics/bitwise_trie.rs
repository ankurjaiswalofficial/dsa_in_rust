//! Bitwise trie for maximum XOR queries.
//!
//! A bitwise trie stores integers one bit at a time instead of one character at a time. The usual
//! learning problem is maximum XOR: to maximize the XOR with a query value, we greedily try to walk
//! to the opposite bit at each level.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trie_topics::bitwise_trie::BitwiseTrie;
//!
//! let mut trie = BitwiseTrie::default();
//! for value in [3, 10, 5, 25, 2, 8] {
//!     trie.insert(value);
//! }
//! assert_eq!(trie.max_xor(5), 28);
//! ```

#[derive(Debug, Clone, Default)]
struct Node {
    next: [Option<Box<Node>>; 2],
}

#[derive(Debug, Clone, Default)]
pub struct BitwiseTrie {
    root: Node,
}

impl BitwiseTrie {
    pub fn insert(&mut self, value: i32) {
        let mut node = &mut self.root;
        for bit in (0..31).rev() {
            let current = ((value >> bit) & 1) as usize;
            node = node.next[current]
                .get_or_insert_with(|| Box::new(Node::default()))
                .as_mut();
        }
    }

    pub fn max_xor(&self, value: i32) -> i32 {
        let mut node = &self.root;
        let mut answer = 0;
        for bit in (0..31).rev() {
            let current = ((value >> bit) & 1) as usize;
            let preferred = 1 - current;
            if let Some(next) = node.next[preferred].as_deref() {
                answer |= 1 << bit;
                node = next;
            } else if let Some(next) = node.next[current].as_deref() {
                node = next;
            } else {
                break;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::BitwiseTrie;

    #[test]
    fn bitwise_trie_answers_max_xor_queries() {
        let mut trie = BitwiseTrie::default();
        for value in [3, 10, 5, 25, 2, 8] {
            trie.insert(value);
        }
        assert_eq!(trie.max_xor(5), 28);
        assert_eq!(trie.max_xor(0), 25);
    }
}
