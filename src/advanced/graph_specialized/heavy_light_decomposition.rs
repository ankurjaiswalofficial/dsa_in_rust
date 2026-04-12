//! Heavy-light decomposition.
//!
//! Heavy-light decomposition splits a tree into heavy paths so path queries can be answered with a
//! segment tree or Fenwick tree over the decomposed order.
//!
//! This implementation focuses on the decomposition itself: head of the current heavy chain and the
//! position of each node in the flattened order.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::heavy_light_decomposition::HeavyLightDecomposition;
//!
//! let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let hld = HeavyLightDecomposition::new(&tree, 0);
//! assert_eq!(hld.position.len(), 4);
//! ```

#[derive(Debug, Clone)]
pub struct HeavyLightDecomposition {
    pub parent: Vec<usize>,
    pub depth: Vec<usize>,
    pub heavy: Vec<Option<usize>>,
    pub head: Vec<usize>,
    pub position: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new(tree: &[Vec<usize>], root: usize) -> Self {
        let n = tree.len();
        let mut parent = vec![root; n];
        let mut depth = vec![0; n];
        let mut heavy = vec![None; n];
        let mut size = vec![0; n];

        fn dfs(
            node: usize,
            par: usize,
            tree: &[Vec<usize>],
            parent: &mut [usize],
            depth: &mut [usize],
            heavy: &mut [Option<usize>],
            size: &mut [usize],
        ) -> usize {
            parent[node] = par;
            size[node] = 1;
            let mut best_size = 0;
            for &next in &tree[node] {
                if next == par {
                    continue;
                }
                depth[next] = depth[node] + 1;
                let child_size = dfs(next, node, tree, parent, depth, heavy, size);
                size[node] += child_size;
                if child_size > best_size {
                    best_size = child_size;
                    heavy[node] = Some(next);
                }
            }
            size[node]
        }

        dfs(root, root, tree, &mut parent, &mut depth, &mut heavy, &mut size);
        let mut head = vec![root; n];
        let mut position = vec![0; n];
        let mut current = 0;
        fn decompose(
            node: usize,
            par: usize,
            current_head: usize,
            tree: &[Vec<usize>],
            heavy: &[Option<usize>],
            head: &mut [usize],
            position: &mut [usize],
            current: &mut usize,
        ) {
            head[node] = current_head;
            position[node] = *current;
            *current += 1;
            if let Some(heavy_child) = heavy[node] {
                decompose(heavy_child, node, current_head, tree, heavy, head, position, current);
            }
            for &next in &tree[node] {
                if next != par && Some(next) != heavy[node] {
                    decompose(next, node, next, tree, heavy, head, position, current);
                }
            }
        }
        decompose(root, root, root, tree, &heavy, &mut head, &mut position, &mut current);
        Self { parent, depth, heavy, head, position }
    }
}

#[cfg(test)]
mod tests {
    use super::HeavyLightDecomposition;

    #[test]
    fn heavy_light_decomposition_assigns_positions() {
        let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let hld = HeavyLightDecomposition::new(&tree, 0);
        assert_eq!(hld.position.len(), 4);
    }
}

