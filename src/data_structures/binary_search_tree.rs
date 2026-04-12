//! Binary search tree.
//!
//! A BST stores values so that everything in the left subtree is smaller and everything in the
//! right subtree is larger. This supports search and insertion in logarithmic time on balanced
//! inputs, but linear time on a skewed tree.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::binary_search_tree::BinarySearchTree;
//!
//! let mut tree = BinarySearchTree::new();
//! tree.insert(5);
//! tree.insert(3);
//! tree.insert(7);
//! assert!(tree.contains(&3));
//! assert_eq!(tree.inorder(), vec![3, 5, 7]);
//! ```

type TreeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
}

#[derive(Debug, Clone, Default)]
pub struct BinarySearchTree<T> {
    root: TreeLink<T>,
    len: usize,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn insert(&mut self, value: T) {
        if Self::insert_node(&mut self.root, value) {
            self.len += 1;
        }
    }

    fn insert_node(node: &mut TreeLink<T>, value: T) -> bool {
        match node {
            Some(current) => {
                if value < current.value {
                    Self::insert_node(&mut current.left, value)
                } else if value > current.value {
                    Self::insert_node(&mut current.right, value)
                } else {
                    false
                }
            }
            None => {
                *node = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
                true
            }
        }
    }

    pub fn contains(&self, target: &T) -> bool {
        let mut current = self.root.as_deref();
        while let Some(node) = current {
            if target < &node.value {
                current = node.left.as_deref();
            } else if target > &node.value {
                current = node.right.as_deref();
            } else {
                return true;
            }
        }
        false
    }

    pub fn inorder(&self) -> Vec<T> {
        let mut values = Vec::with_capacity(self.len);
        Self::inorder_node(&self.root, &mut values);
        values
    }

    fn inorder_node(node: &TreeLink<T>, values: &mut Vec<T>) {
        if let Some(current) = node {
            Self::inorder_node(&current.left, values);
            values.push(current.value.clone());
            Self::inorder_node(&current.right, values);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn bst_preserves_sorted_order() {
        let mut tree = BinarySearchTree::new();
        for value in [5, 2, 8, 1, 3, 7, 9] {
            tree.insert(value);
        }
        assert_eq!(tree.inorder(), vec![1, 2, 3, 5, 7, 8, 9]);
        assert!(tree.contains(&7));
        assert!(!tree.contains(&6));
    }
}
