//! Binary lifting.
//!
//! Binary lifting preprocesses `2^k` ancestors so ancestor and LCA queries can be answered in
//! logarithmic time.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::binary_lifting::BinaryLifting;
//!
//! let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let jump = BinaryLifting::new(&tree, 0);
//! assert_eq!(jump.kth_ancestor(3, 1), 1);
//! ```

#[derive(Debug, Clone)]
pub struct BinaryLifting {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl BinaryLifting {
    pub fn new(tree: &[Vec<usize>], root: usize) -> Self {
        let n = tree.len();
        let mut log = 1;
        while (1usize << log) <= n.max(1) {
            log += 1;
        }
        let mut up = vec![vec![root; n]; log];
        let mut depth = vec![0; n];
        fn dfs(node: usize, par: usize, tree: &[Vec<usize>], up: &mut [Vec<usize>], depth: &mut [usize]) {
            up[0][node] = par;
            for &next in &tree[node] {
                if next != par {
                    depth[next] = depth[node] + 1;
                    dfs(next, node, tree, up, depth);
                }
            }
        }
        dfs(root, root, tree, &mut up, &mut depth);
        for k in 1..log {
            for v in 0..n {
                up[k][v] = up[k - 1][up[k - 1][v]];
            }
        }
        Self { up, depth }
    }

    pub fn kth_ancestor(&self, mut node: usize, k: usize) -> usize {
        for bit in 0..self.up.len() {
            if (k >> bit) & 1 == 1 {
                node = self.up[bit][node];
            }
        }
        node
    }

    pub fn depth(&self, node: usize) -> usize {
        self.depth[node]
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryLifting;

    #[test]
    fn binary_lifting_answers_ancestor_queries() {
        let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let jump = BinaryLifting::new(&tree, 0);
        assert_eq!(jump.kth_ancestor(3, 1), 1);
        assert_eq!(jump.kth_ancestor(3, 2), 0);
        assert_eq!(jump.depth(3), 2);
    }
}

