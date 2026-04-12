//! Merge sort tree.
//!
//! A merge sort tree stores the sorted values of each segment, allowing queries such as "how many
//! numbers in `[l, r]` are <= x" in `O(log^2 n)`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::range_queries::merge_sort_tree::MergeSortTree;
//!
//! let tree = MergeSortTree::new(&[5, 1, 2, 6, 3]);
//! assert_eq!(tree.count_less_equal(1, 3, 2), 2);
//! ```

#[derive(Debug, Clone)]
pub struct MergeSortTree {
    len: usize,
    tree: Vec<Vec<i32>>,
}

impl MergeSortTree {
    pub fn new(values: &[i32]) -> Self {
        let len = values.len();
        let tree = vec![Vec::new(); len.max(1) * 4];
        let mut result = Self { len, tree };
        if len > 0 {
            result.build(1, 0, len - 1, values);
        }
        result
    }

    pub fn count_less_equal(&self, left: usize, right: usize, x: i32) -> usize {
        self.query(1, 0, self.len - 1, left, right, x)
    }

    fn build(&mut self, node: usize, left: usize, right: usize, values: &[i32]) {
        if left == right {
            self.tree[node] = vec![values[left]];
            return;
        }
        let mid = (left + right) / 2;
        self.build(node * 2, left, mid, values);
        self.build(node * 2 + 1, mid + 1, right, values);
        self.tree[node] = merge(&self.tree[node * 2], &self.tree[node * 2 + 1]);
    }

    fn query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize, x: i32) -> usize {
        if qr < left || right < ql {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node].partition_point(|&value| value <= x);
        }
        let mid = (left + right) / 2;
        self.query(node * 2, left, mid, ql, qr, x)
            + self.query(node * 2 + 1, mid + 1, right, ql, qr, x)
    }
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    merged
}

#[cfg(test)]
mod tests {
    use super::MergeSortTree;

    #[test]
    fn merge_sort_tree_counts_values_by_threshold() {
        let tree = MergeSortTree::new(&[5, 1, 2, 6, 3]);
        assert_eq!(tree.count_less_equal(1, 3, 2), 2);
    }
}
