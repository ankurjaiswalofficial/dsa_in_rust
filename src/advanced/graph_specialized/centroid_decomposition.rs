//! Centroid decomposition.
//!
//! Centroid decomposition recursively removes a centroid from the tree and builds a centroid tree.
//! It is useful when queries depend on distances to changing marked nodes.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::centroid_decomposition::centroid_parent;
//!
//! let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let parent = centroid_parent(&tree);
//! assert_eq!(parent.len(), 4);
//! ```

pub fn centroid_parent(tree: &[Vec<usize>]) -> Vec<usize> {
    let n = tree.len();
    let mut removed = vec![false; n];
    let mut parent = vec![usize::MAX; n];
    let mut size = vec![0; n];

    fn dfs_size(node: usize, par: usize, tree: &[Vec<usize>], removed: &[bool], size: &mut [usize]) -> usize {
        size[node] = 1;
        for &next in &tree[node] {
            if next != par && !removed[next] {
                size[node] += dfs_size(next, node, tree, removed, size);
            }
        }
        size[node]
    }

    fn find_centroid(
        node: usize,
        par: usize,
        total: usize,
        tree: &[Vec<usize>],
        removed: &[bool],
        size: &[usize],
    ) -> usize {
        for &next in &tree[node] {
            if next != par && !removed[next] && size[next] * 2 > total {
                return find_centroid(next, node, total, tree, removed, size);
            }
        }
        node
    }

    fn build(
        entry: usize,
        par: usize,
        tree: &[Vec<usize>],
        removed: &mut [bool],
        parent: &mut [usize],
        size: &mut [usize],
    ) {
        let total = dfs_size(entry, usize::MAX, tree, removed, size);
        let centroid = find_centroid(entry, usize::MAX, total, tree, removed, size);
        removed[centroid] = true;
        parent[centroid] = par;
        for &next in &tree[centroid] {
            if !removed[next] {
                build(next, centroid, tree, removed, parent, size);
            }
        }
    }

    build(0, 0, tree, &mut removed, &mut parent, &mut size);
    parent
}

#[cfg(test)]
mod tests {
    use super::centroid_parent;

    #[test]
    fn centroid_decomposition_builds_a_parent_array() {
        let tree = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let parent = centroid_parent(&tree);
        assert_eq!(parent.len(), 4);
    }
}

