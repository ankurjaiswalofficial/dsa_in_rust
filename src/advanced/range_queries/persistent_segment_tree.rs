//! Persistent segment tree.
//!
//! A persistent segment tree preserves every historical version after an update. That makes it
//! useful for offline k-th order statistics and time-travel style queries.
//!
//! # Example
//!
//! ```
//! use project1::advanced::range_queries::persistent_segment_tree::PersistentSegmentTree;
//!
//! let mut tree = PersistentSegmentTree::new(&[1, 2, 3]);
//! let v1 = tree.update(0, 1, 10);
//! assert_eq!(tree.query(0, 0, 2), 6);
//! assert_eq!(tree.query(v1, 0, 2), 14);
//! ```

#[derive(Debug, Clone)]
struct Node {
    sum: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct PersistentSegmentTree {
    versions: Vec<Box<Node>>,
    len: usize,
}

impl PersistentSegmentTree {
    pub fn new(values: &[i64]) -> Self {
        let root = build(values, 0, values.len().saturating_sub(1));
        Self { versions: vec![root], len: values.len() }
    }

    pub fn update(&mut self, version: usize, index: usize, value: i64) -> usize {
        let root = update(&self.versions[version], 0, self.len - 1, index, value);
        self.versions.push(root);
        self.versions.len() - 1
    }

    pub fn query(&self, version: usize, left: usize, right: usize) -> i64 {
        query(&self.versions[version], 0, self.len - 1, left, right)
    }
}

fn build(values: &[i64], left: usize, right: usize) -> Box<Node> {
    if values.is_empty() {
        return Box::new(Node { sum: 0, left: None, right: None });
    }
    if left == right {
        return Box::new(Node { sum: values[left], left: None, right: None });
    }
    let mid = (left + right) / 2;
    let left_node = build(values, left, mid);
    let right_node = build(values, mid + 1, right);
    Box::new(Node {
        sum: left_node.sum + right_node.sum,
        left: Some(left_node),
        right: Some(right_node),
    })
}

fn update(node: &Node, left: usize, right: usize, index: usize, value: i64) -> Box<Node> {
    if left == right {
        return Box::new(Node { sum: value, left: None, right: None });
    }
    let mid = (left + right) / 2;
    let (left_node, right_node) = if index <= mid {
        (
            update(node.left.as_deref().expect("left child"), left, mid, index, value),
            node.right.clone().expect("right child"),
        )
    } else {
        (
            node.left.clone().expect("left child"),
            update(node.right.as_deref().expect("right child"), mid + 1, right, index, value),
        )
    };
    Box::new(Node {
        sum: left_node.sum + right_node.sum,
        left: Some(left_node),
        right: Some(right_node),
    })
}

fn query(node: &Node, left: usize, right: usize, ql: usize, qr: usize) -> i64 {
    if qr < left || right < ql {
        return 0;
    }
    if ql <= left && right <= qr {
        return node.sum;
    }
    let mid = (left + right) / 2;
    query(node.left.as_deref().expect("left child"), left, mid, ql, qr)
        + query(node.right.as_deref().expect("right child"), mid + 1, right, ql, qr)
}

#[cfg(test)]
mod tests {
    use super::PersistentSegmentTree;

    #[test]
    fn persistent_segment_tree_preserves_old_versions() {
        let mut tree = PersistentSegmentTree::new(&[1, 2, 3]);
        let v1 = tree.update(0, 1, 10);
        assert_eq!(tree.query(0, 0, 2), 6);
        assert_eq!(tree.query(v1, 0, 2), 14);
    }
}
