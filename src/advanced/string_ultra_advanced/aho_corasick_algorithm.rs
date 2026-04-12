//! Aho-Corasick algorithm.
//!
//! Aho-Corasick matches many patterns simultaneously by combining a trie with failure links.
//!
//! # Example
//!
//! ```
//! use project1::advanced::string_ultra_advanced::aho_corasick_algorithm::count_pattern_hits;
//!
//! assert_eq!(count_pattern_hits(&["he", "she", "his", "hers"], "ushers"), 3);
//! ```

use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Node {
    next: HashMap<u8, usize>,
    fail: usize,
    output: usize,
}

pub fn count_pattern_hits(patterns: &[&str], text: &str) -> usize {
    let mut nodes = vec![Node::default()];
    for pattern in patterns {
        let mut state = 0;
        for &byte in pattern.as_bytes() {
            let next = if let Some(&child) = nodes[state].next.get(&byte) {
                child
            } else {
                nodes.push(Node::default());
                let child = nodes.len() - 1;
                nodes[state].next.insert(byte, child);
                child
            };
            state = next;
        }
        nodes[state].output += 1;
    }
    let mut queue = VecDeque::new();
    let root_children: Vec<usize> = nodes[0].next.values().copied().collect();
    for child in root_children {
        queue.push_back(child);
    }
    while let Some(state) = queue.pop_front() {
        let edges: Vec<(u8, usize)> = nodes[state].next.iter().map(|(&c, &to)| (c, to)).collect();
        for (ch, next) in edges {
            queue.push_back(next);
            let mut fail = nodes[state].fail;
            while fail > 0 && !nodes[fail].next.contains_key(&ch) {
                fail = nodes[fail].fail;
            }
            nodes[next].fail = nodes[fail].next.get(&ch).copied().unwrap_or(0);
            nodes[next].output += nodes[nodes[next].fail].output;
        }
    }
    let mut state = 0;
    let mut answer = 0;
    for &byte in text.as_bytes() {
        while state > 0 && !nodes[state].next.contains_key(&byte) {
            state = nodes[state].fail;
        }
        state = nodes[state].next.get(&byte).copied().unwrap_or(0);
        answer += nodes[state].output;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::count_pattern_hits;

    #[test]
    fn aho_corasick_counts_multiple_pattern_matches() {
        assert_eq!(count_pattern_hits(&["he", "she", "his", "hers"], "ushers"), 3);
    }
}

