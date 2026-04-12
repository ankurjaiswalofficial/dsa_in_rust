//! Segment tree for range sums with point updates.
//!
//! Segment trees are more flexible than Fenwick trees because they can support many range
//! operations and custom merge functions.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::segment_tree::SegmentTree;
//!
//! let mut tree = SegmentTree::from_slice(&[2, 1, 3, 4]);
//! assert_eq!(tree.query(1, 3), 8);
//! tree.update(2, 10);
//! assert_eq!(tree.query(1, 3), 15);
//! ```

#[derive(Debug, Clone)]
pub struct SegmentTree {
    len: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn from_slice(values: &[i64]) -> Self {
        let len = values.len();
        let tree = vec![0; len.max(1) * 4];
        let mut result = Self { len, tree };
        if len > 0 {
            result.build(1, 0, len - 1, values);
        }
        result
    }

    fn build(&mut self, node: usize, left: usize, right: usize, values: &[i64]) {
        if left == right {
            self.tree[node] = values[left];
            return;
        }
        let mid = (left + right) / 2;
        self.build(node * 2, left, mid, values);
        self.build(node * 2 + 1, mid + 1, right, values);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    pub fn update(&mut self, index: usize, value: i64) {
        if self.len == 0 {
            return;
        }
        self.update_inner(1, 0, self.len - 1, index, value);
    }

    fn update_inner(&mut self, node: usize, left: usize, right: usize, index: usize, value: i64) {
        if left == right {
            self.tree[node] = value;
            return;
        }
        let mid = (left + right) / 2;
        if index <= mid {
            self.update_inner(node * 2, left, mid, index, value);
        } else {
            self.update_inner(node * 2 + 1, mid + 1, right, index, value);
        }
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    pub fn query(&self, query_left: usize, query_right: usize) -> i64 {
        if self.len == 0 || query_left > query_right {
            return 0;
        }
        self.query_inner(1, 0, self.len - 1, query_left, query_right)
    }

    fn query_inner(
        &self,
        node: usize,
        left: usize,
        right: usize,
        query_left: usize,
        query_right: usize,
    ) -> i64 {
        if query_left <= left && right <= query_right {
            return self.tree[node];
        }
        if right < query_left || query_right < left {
            return 0;
        }
        let mid = (left + right) / 2;
        self.query_inner(node * 2, left, mid, query_left, query_right)
            + self.query_inner(node * 2 + 1, mid + 1, right, query_left, query_right)
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;

    #[test]
    fn segment_tree_supports_query_and_update() {
        let mut tree = SegmentTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(tree.query(1, 3), 9);
        tree.update(2, 10);
        assert_eq!(tree.query(1, 3), 16);
    }
}
