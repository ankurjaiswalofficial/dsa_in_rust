//! Segment tree with lazy propagation for range add and range sum.
//!
//! Plain segment trees handle point updates well, but repeated range updates can become expensive
//! if we push every change down to every affected leaf immediately. Lazy propagation postpones that
//! work by storing a pending update in an internal node and applying it only when a query or deeper
//! traversal actually needs the children.
//!
//! This implementation supports:
//! - `range_add(left, right, delta)`
//! - `range_sum(left, right)`
//!
//! # Example
//!
//! ```
//! use project1::advanced::trees::lazy_propagation::LazySegmentTree;
//!
//! let mut tree = LazySegmentTree::from_slice(&[1, 2, 3, 4, 5]);
//! tree.range_add(1, 3, 10);
//! assert_eq!(tree.range_sum(0, 4), 45);
//! assert_eq!(tree.range_sum(1, 3), 39);
//! ```

#[derive(Debug, Clone)]
pub struct LazySegmentTree {
    len: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn from_slice(values: &[i64]) -> Self {
        let len = values.len();
        let mut result = Self {
            len,
            tree: vec![0; len.max(1) * 4],
            lazy: vec![0; len.max(1) * 4],
        };
        if len > 0 {
            result.build(1, 0, len - 1, values);
        }
        result
    }

    pub fn range_add(&mut self, left: usize, right: usize, delta: i64) {
        if self.len == 0 || left > right {
            return;
        }
        self.range_add_inner(1, 0, self.len - 1, left, right, delta);
    }

    pub fn range_sum(&mut self, left: usize, right: usize) -> i64 {
        if self.len == 0 || left > right {
            return 0;
        }
        self.range_sum_inner(1, 0, self.len - 1, left, right)
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

    fn apply(&mut self, node: usize, left: usize, right: usize, delta: i64) {
        self.tree[node] += delta * (right as i64 - left as i64 + 1);
        self.lazy[node] += delta;
    }

    fn push(&mut self, node: usize, left: usize, right: usize) {
        if self.lazy[node] == 0 || left == right {
            return;
        }
        let mid = (left + right) / 2;
        let delta = self.lazy[node];
        self.apply(node * 2, left, mid, delta);
        self.apply(node * 2 + 1, mid + 1, right, delta);
        self.lazy[node] = 0;
    }

    fn range_add_inner(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        query_left: usize,
        query_right: usize,
        delta: i64,
    ) {
        if query_right < left || right < query_left {
            return;
        }
        if query_left <= left && right <= query_right {
            self.apply(node, left, right, delta);
            return;
        }
        self.push(node, left, right);
        let mid = (left + right) / 2;
        self.range_add_inner(node * 2, left, mid, query_left, query_right, delta);
        self.range_add_inner(node * 2 + 1, mid + 1, right, query_left, query_right, delta);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn range_sum_inner(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        query_left: usize,
        query_right: usize,
    ) -> i64 {
        if query_right < left || right < query_left {
            return 0;
        }
        if query_left <= left && right <= query_right {
            return self.tree[node];
        }
        self.push(node, left, right);
        let mid = (left + right) / 2;
        self.range_sum_inner(node * 2, left, mid, query_left, query_right)
            + self.range_sum_inner(node * 2 + 1, mid + 1, right, query_left, query_right)
    }
}

#[cfg(test)]
mod tests {
    use super::LazySegmentTree;

    #[test]
    fn lazy_segment_tree_supports_range_updates_and_queries() {
        let mut tree = LazySegmentTree::from_slice(&[1, 2, 3, 4, 5]);
        tree.range_add(1, 3, 10);
        assert_eq!(tree.range_sum(0, 4), 45);
        assert_eq!(tree.range_sum(1, 3), 39);
    }

    #[test]
    fn lazy_segment_tree_handles_multiple_updates() {
        let mut tree = LazySegmentTree::from_slice(&[0, 0, 0, 0]);
        tree.range_add(0, 2, 5);
        tree.range_add(1, 3, 2);
        assert_eq!(tree.range_sum(0, 0), 5);
        assert_eq!(tree.range_sum(1, 2), 14);
        assert_eq!(tree.range_sum(3, 3), 2);
    }
}
