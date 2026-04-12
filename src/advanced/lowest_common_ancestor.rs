//! Lowest common ancestor with binary lifting.
//!
//! LCA answers the question: what is the deepest shared ancestor of two nodes in a rooted tree?
//!
//! # Example
//!
//! ```
//! use project1::advanced::lowest_common_ancestor::LowestCommonAncestor;
//!
//! let tree = vec![
//!     vec![1, 2],
//!     vec![0, 3, 4],
//!     vec![0],
//!     vec![1],
//!     vec![1],
//! ];
//! let lca = LowestCommonAncestor::new(&tree, 0);
//! assert_eq!(lca.query(3, 4), 1);
//! assert_eq!(lca.query(2, 4), 0);
//! ```

#[derive(Debug, Clone)]
pub struct LowestCommonAncestor {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LowestCommonAncestor {
    pub fn new(tree: &[Vec<usize>], root: usize) -> Self {
        let n = tree.len();
        let mut log = 1;
        while (1usize << log) <= n.max(1) {
            log += 1;
        }
        let mut up = vec![vec![root; n]; log];
        let mut depth = vec![0; n];

        fn dfs(
            node: usize,
            parent: usize,
            tree: &[Vec<usize>],
            up: &mut [Vec<usize>],
            depth: &mut [usize],
        ) {
            up[0][node] = parent;
            for &next in &tree[node] {
                if next != parent {
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

    pub fn query(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }

        let diff = self.depth[a] - self.depth[b];
        for k in 0..self.up.len() {
            if (diff >> k) & 1 == 1 {
                a = self.up[k][a];
            }
        }

        if a == b {
            return a;
        }

        for k in (0..self.up.len()).rev() {
            if self.up[k][a] != self.up[k][b] {
                a = self.up[k][a];
                b = self.up[k][b];
            }
        }
        self.up[0][a]
    }
}

#[cfg(test)]
mod tests {
    use super::LowestCommonAncestor;

    #[test]
    fn lca_answers_tree_queries() {
        let tree = vec![
            vec![1, 2],
            vec![0, 3, 4],
            vec![0],
            vec![1],
            vec![1, 5],
            vec![4],
        ];
        let lca = LowestCommonAncestor::new(&tree, 0);
        assert_eq!(lca.query(3, 5), 1);
        assert_eq!(lca.query(2, 5), 0);
    }
}
