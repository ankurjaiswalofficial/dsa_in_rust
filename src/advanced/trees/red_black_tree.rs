//! Left-leaning red-black tree.
//!
//! Red-black trees are balanced binary search trees that encode 2-3 tree structure with colored
//! links. This file implements insertion for the left-leaning red-black variant, which keeps the
//! balancing logic compact enough to study directly.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trees::red_black_tree::RedBlackTree;
//!
//! let mut tree = RedBlackTree::default();
//! for value in [10, 20, 30, 15, 25] {
//!     tree.insert(value);
//! }
//! assert!(tree.contains(15));
//! assert_eq!(tree.inorder(), vec![10, 15, 20, 25, 30]);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    color: Color,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, Clone, Default)]
pub struct RedBlackTree {
    root: Option<Box<Node>>,
}

impl RedBlackTree {
    pub fn insert(&mut self, key: i64) {
        self.root = Some(insert(self.root.take(), key));
        if let Some(root) = self.root.as_mut() {
            root.color = Color::Black;
        }
    }

    pub fn contains(&self, key: i64) -> bool {
        let mut current = self.root.as_deref();
        while let Some(node) = current {
            if key == node.key {
                return true;
            }
            current = if key < node.key {
                node.left.as_deref()
            } else {
                node.right.as_deref()
            };
        }
        false
    }

    pub fn inorder(&self) -> Vec<i64> {
        fn dfs(node: &Option<Box<Node>>, out: &mut Vec<i64>) {
            if let Some(node) = node {
                dfs(&node.left, out);
                out.push(node.key);
                dfs(&node.right, out);
            }
        }
        let mut out = Vec::new();
        dfs(&self.root, &mut out);
        out
    }

    pub fn is_valid(&self) -> bool {
        fn dfs(node: &Option<Box<Node>>) -> Option<usize> {
            let Some(node) = node else {
                return Some(1);
            };
            if is_red_ref(node.right.as_deref()) {
                return None;
            }
            if is_red_ref(node.left.as_deref()) && is_red_ref(node.left.as_deref().and_then(|n| n.left.as_deref())) {
                return None;
            }
            let left = dfs(&node.left)?;
            let right = dfs(&node.right)?;
            if left != right {
                return None;
            }
            Some(left + usize::from(node.color == Color::Black))
        }

        self.root
            .as_ref()
            .is_none_or(|root| root.color == Color::Black)
            && dfs(&self.root).is_some()
    }
}

fn is_red_ref(node: Option<&Node>) -> bool {
    matches!(node.map(|node| node.color), Some(Color::Red))
}

fn rotate_left(mut node: Box<Node>) -> Box<Node> {
    let mut right = node.right.take().expect("right child required");
    node.right = right.left.take();
    right.left = Some(node);
    right.color = right.left.as_ref().expect("left child exists").color;
    right.left.as_mut().expect("left child exists").color = Color::Red;
    right
}

fn rotate_right(mut node: Box<Node>) -> Box<Node> {
    let mut left = node.left.take().expect("left child required");
    node.left = left.right.take();
    left.right = Some(node);
    left.color = left.right.as_ref().expect("right child exists").color;
    left.right.as_mut().expect("right child exists").color = Color::Red;
    left
}

fn flip_colors(node: &mut Box<Node>) {
    node.color = match node.color {
        Color::Red => Color::Black,
        Color::Black => Color::Red,
    };
    if let Some(left) = node.left.as_mut() {
        left.color = match left.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
    if let Some(right) = node.right.as_mut() {
        right.color = match right.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
}

fn insert(node: Option<Box<Node>>, key: i64) -> Box<Node> {
    let mut node = match node {
        None => {
            return Box::new(Node {
                key,
                color: Color::Red,
                left: None,
                right: None,
            })
        }
        Some(node) => node,
    };

    if key < node.key {
        node.left = Some(insert(node.left.take(), key));
    } else if key > node.key {
        node.right = Some(insert(node.right.take(), key));
    }

    if is_red_ref(node.right.as_deref()) && !is_red_ref(node.left.as_deref()) {
        node = rotate_left(node);
    }
    if is_red_ref(node.left.as_deref()) && is_red_ref(node.left.as_deref().and_then(|n| n.left.as_deref())) {
        node = rotate_right(node);
    }
    if is_red_ref(node.left.as_deref()) && is_red_ref(node.right.as_deref()) {
        flip_colors(&mut node);
    }

    node
}

#[cfg(test)]
mod tests {
    use super::RedBlackTree;

    #[test]
    fn red_black_tree_inserts_and_preserves_sorted_order() {
        let mut tree = RedBlackTree::default();
        for value in [10, 20, 30, 15, 25] {
            tree.insert(value);
        }
        assert_eq!(tree.inorder(), vec![10, 15, 20, 25, 30]);
    }

    #[test]
    fn red_black_tree_maintains_invariants_after_insertions() {
        let mut tree = RedBlackTree::default();
        for value in [7, 3, 18, 10, 22, 8, 11, 26] {
            tree.insert(value);
        }
        assert!(tree.contains(11));
        assert!(tree.is_valid());
    }
}
