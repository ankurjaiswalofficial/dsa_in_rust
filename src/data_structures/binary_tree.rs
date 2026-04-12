//! Binary tree utilities.
//!
//! This module focuses on structural tree reasoning rather than the BST ordering
//! invariant. It covers traversals and common interview-style binary-tree
//! queries.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::binary_tree::{BinaryTreeNode, inorder_values, level_order_values, lowest_common_ancestor};
//!
//! let root = BinaryTreeNode::sample_tree();
//! assert_eq!(inorder_values(&root), vec![4, 2, 5, 1, 3]);
//! assert_eq!(level_order_values(&root), vec![vec![1], vec![2, 3], vec![4, 5]]);
//! assert_eq!(lowest_common_ancestor(&root, 4, 5), Some(2));
//! ```

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryTreeNode {
    pub value: i32,
    pub left: Option<Box<BinaryTreeNode>>,
    pub right: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    /// Small reusable tree for examples and tests.
    pub fn sample_tree() -> Option<Box<BinaryTreeNode>> {
        Some(Box::new(BinaryTreeNode {
            value: 1,
            left: Some(Box::new(BinaryTreeNode {
                value: 2,
                left: Some(Box::new(BinaryTreeNode::new(4))),
                right: Some(Box::new(BinaryTreeNode::new(5))),
            })),
            right: Some(Box::new(BinaryTreeNode::new(3))),
        }))
    }
}

pub fn inorder_values(root: &Option<Box<BinaryTreeNode>>) -> Vec<i32> {
    fn dfs(node: &Option<Box<BinaryTreeNode>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            dfs(&node.left, out);
            out.push(node.value);
            dfs(&node.right, out);
        }
    }

    let mut out = Vec::new();
    dfs(root, &mut out);
    out
}

pub fn preorder_values(root: &Option<Box<BinaryTreeNode>>) -> Vec<i32> {
    fn dfs(node: &Option<Box<BinaryTreeNode>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            out.push(node.value);
            dfs(&node.left, out);
            dfs(&node.right, out);
        }
    }

    let mut out = Vec::new();
    dfs(root, &mut out);
    out
}

pub fn postorder_values(root: &Option<Box<BinaryTreeNode>>) -> Vec<i32> {
    fn dfs(node: &Option<Box<BinaryTreeNode>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            dfs(&node.left, out);
            dfs(&node.right, out);
            out.push(node.value);
        }
    }

    let mut out = Vec::new();
    dfs(root, &mut out);
    out
}

pub fn level_order_values(root: &Option<Box<BinaryTreeNode>>) -> Vec<Vec<i32>> {
    let Some(root) = root.as_deref() else {
        return Vec::new();
    };

    let mut queue = VecDeque::from([root]);
    let mut levels = Vec::new();
    while !queue.is_empty() {
        let size = queue.len();
        let mut level = Vec::with_capacity(size);
        for _ in 0..size {
            let node = queue.pop_front().expect("queue has enough nodes for this level");
            level.push(node.value);
            if let Some(left) = node.left.as_deref() {
                queue.push_back(left);
            }
            if let Some(right) = node.right.as_deref() {
                queue.push_back(right);
            }
        }
        levels.push(level);
    }
    levels
}

pub fn height(root: &Option<Box<BinaryTreeNode>>) -> i32 {
    match root {
        Some(node) => 1 + height(&node.left).max(height(&node.right)),
        None => 0,
    }
}

pub fn is_balanced(root: &Option<Box<BinaryTreeNode>>) -> bool {
    fn dfs(node: &Option<Box<BinaryTreeNode>>) -> Option<i32> {
        let Some(node) = node else {
            return Some(0);
        };
        let left = dfs(&node.left)?;
        let right = dfs(&node.right)?;
        ((left - right).abs() <= 1).then_some(1 + left.max(right))
    }
    dfs(root).is_some()
}

pub fn has_path_sum(root: &Option<Box<BinaryTreeNode>>, target: i32) -> bool {
    match root {
        Some(node) if node.left.is_none() && node.right.is_none() => node.value == target,
        Some(node) => {
            let remaining = target - node.value;
            has_path_sum(&node.left, remaining) || has_path_sum(&node.right, remaining)
        }
        None => false,
    }
}

pub fn diameter(root: &Option<Box<BinaryTreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<BinaryTreeNode>>, best: &mut i32) -> i32 {
        let Some(node) = node else {
            return 0;
        };
        let left = dfs(&node.left, best);
        let right = dfs(&node.right, best);
        *best = (*best).max(left + right);
        1 + left.max(right)
    }

    let mut best = 0;
    dfs(root, &mut best);
    best
}

pub fn lowest_common_ancestor(
    root: &Option<Box<BinaryTreeNode>>,
    first: i32,
    second: i32,
) -> Option<i32> {
    fn dfs(node: &Option<Box<BinaryTreeNode>>, first: i32, second: i32) -> Option<i32> {
        let Some(node) = node else {
            return None;
        };
        if node.value == first || node.value == second {
            return Some(node.value);
        }
        let left = dfs(&node.left, first, second);
        let right = dfs(&node.right, first, second);
        match (left, right) {
            (Some(_), Some(_)) => Some(node.value),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        }
    }

    dfs(root, first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traversals_and_levels_work() {
        let root = BinaryTreeNode::sample_tree();
        assert_eq!(inorder_values(&root), vec![4, 2, 5, 1, 3]);
        assert_eq!(preorder_values(&root), vec![1, 2, 4, 5, 3]);
        assert_eq!(postorder_values(&root), vec![4, 5, 2, 3, 1]);
        assert_eq!(level_order_values(&root), vec![vec![1], vec![2, 3], vec![4, 5]]);
    }

    #[test]
    fn structural_queries_work() {
        let root = BinaryTreeNode::sample_tree();
        assert_eq!(height(&root), 3);
        assert!(is_balanced(&root));
        assert!(has_path_sum(&root, 7));
        assert_eq!(diameter(&root), 3);
        assert_eq!(lowest_common_ancestor(&root, 4, 5), Some(2));
        assert_eq!(lowest_common_ancestor(&root, 4, 3), Some(1));
    }
}
