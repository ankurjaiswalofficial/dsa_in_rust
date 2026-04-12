//! Iterator design.
//!
//! Designing an iterator means explicitly modeling traversal state. The example below is an in-order
//! iterator over a binary tree represented by nodes in a vector.
//!
//! # Example
//!
//! ```
//! use project1::advanced::practical_topics::iterator_design::{InorderIter, Node};
//!
//! let nodes = vec![
//!     Node { value: 2, left: Some(1), right: Some(2) },
//!     Node { value: 1, left: None, right: None },
//!     Node { value: 3, left: None, right: None },
//! ];
//! let values: Vec<_> = InorderIter::new(&nodes, Some(0)).collect();
//! assert_eq!(values, vec![1, 2, 3]);
//! ```

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub value: i32,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

pub struct InorderIter<'a> {
    nodes: &'a [Node],
    stack: Vec<usize>,
    current: Option<usize>,
}

impl<'a> InorderIter<'a> {
    pub fn new(nodes: &'a [Node], root: Option<usize>) -> Self {
        Self {
            nodes,
            stack: Vec::new(),
            current: root,
        }
    }
}

impl<'a> Iterator for InorderIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.current {
            self.stack.push(index);
            self.current = self.nodes[index].left;
        }
        let index = self.stack.pop()?;
        self.current = self.nodes[index].right;
        Some(self.nodes[index].value)
    }
}

#[cfg(test)]
mod tests {
    use super::{InorderIter, Node};

    #[test]
    fn custom_iterator_traverses_inorder() {
        let nodes = vec![
            Node { value: 2, left: Some(1), right: Some(2) },
            Node { value: 1, left: None, right: None },
            Node { value: 3, left: None, right: None },
        ];
        let values: Vec<_> = InorderIter::new(&nodes, Some(0)).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }
}

