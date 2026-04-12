//! AVL tree with insertion and membership checks.
//!
//! AVL trees are self-balancing binary search trees. After every insertion or deletion, the tree
//! restores the height difference between a node's left and right subtree so that it stays in
//! `[-1, 1]`. That balancing guarantee keeps search, insertion, and deletion in `O(log n)`.
//!
//! The educational flow in this file is:
//! 1. Define the node and tree structure.
//! 2. Insert keys like a normal BST.
//! 3. Recompute heights and rotate when a subtree becomes unbalanced.
//! 4. Verify the result with simple unit tests.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trees::avl_tree::AvlTree;
//!
//! let mut tree = AvlTree::default();
//! for value in [30, 20, 10, 25, 40, 50] {
//!     tree.insert(value);
//! }
//!
//! assert!(tree.contains(25));
//! assert!(!tree.contains(99));
//! assert_eq!(tree.inorder(), vec![10, 20, 25, 30, 40, 50]);
//! ```

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i64) -> Box<Self> {
        Box::new(Self {
            key,
            height: 1,
            left: None,
            right: None,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct AvlTree {
    root: Option<Box<Node>>,
}

impl AvlTree {
    pub fn insert(&mut self, key: i64) {
        self.root = Some(insert(self.root.take(), key));
    }

    pub fn contains(&self, key: i64) -> bool {
        let mut cursor = self.root.as_deref();
        while let Some(node) = cursor {
            if key == node.key {
                return true;
            }
            cursor = if key < node.key {
                node.left.as_deref()
            } else {
                node.right.as_deref()
            };
        }
        false
    }

    pub fn inorder(&self) -> Vec<i64> {
        fn walk(node: &Option<Box<Node>>, out: &mut Vec<i64>) {
            if let Some(node) = node {
                walk(&node.left, out);
                out.push(node.key);
                walk(&node.right, out);
            }
        }

        let mut out = Vec::new();
        walk(&self.root, &mut out);
        out
    }
}

fn height(node: &Option<Box<Node>>) -> i32 {
    node.as_ref().map_or(0, |node| node.height)
}

fn update(node: &mut Box<Node>) {
    node.height = 1 + height(&node.left).max(height(&node.right));
}

fn balance_factor(node: &Node) -> i32 {
    height(&node.left) - height(&node.right)
}

fn rotate_right(mut y: Box<Node>) -> Box<Node> {
    let mut x = y.left.take().expect("left child required");
    y.left = x.right.take();
    update(&mut y);
    x.right = Some(y);
    update(&mut x);
    x
}

fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    let mut y = x.right.take().expect("right child required");
    x.right = y.left.take();
    update(&mut x);
    y.left = Some(x);
    update(&mut y);
    y
}

fn rebalance(mut node: Box<Node>) -> Box<Node> {
    update(&mut node);
    let bf = balance_factor(&node);
    if bf > 1 {
        if balance_factor(node.left.as_deref().expect("left child required")) < 0 {
            let left = node.left.take().map(rotate_left);
            node.left = left;
        }
        return rotate_right(node);
    }
    if bf < -1 {
        if balance_factor(node.right.as_deref().expect("right child required")) > 0 {
            let right = node.right.take().map(rotate_right);
            node.right = right;
        }
        return rotate_left(node);
    }
    node
}

fn insert(node: Option<Box<Node>>, key: i64) -> Box<Node> {
    match node {
        None => Node::new(key),
        Some(mut node) => {
            if key < node.key {
                node.left = Some(insert(node.left.take(), key));
            } else if key > node.key {
                node.right = Some(insert(node.right.take(), key));
            }
            rebalance(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AvlTree;

    #[test]
    fn avl_tree_keeps_sorted_order_after_rotations() {
        let mut tree = AvlTree::default();
        for value in [30, 20, 10, 25, 40, 50] {
            tree.insert(value);
        }
        assert_eq!(tree.inorder(), vec![10, 20, 25, 30, 40, 50]);
    }

    #[test]
    fn avl_tree_membership_queries_work() {
        let mut tree = AvlTree::default();
        for value in [8, 3, 10, 1, 6, 14] {
            tree.insert(value);
        }
        assert!(tree.contains(6));
        assert!(!tree.contains(7));
    }
}
