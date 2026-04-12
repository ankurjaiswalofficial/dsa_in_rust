//! Euler tour technique.
//!
//! An Euler tour records entry and exit events while traversing a tree. It turns subtree queries
//! into range queries on the tour order.
//!
//! # Example
//!
//! ```
//! use project1::advanced::tree_extensions::euler_tour_technique::euler_tour;
//!
//! let tree = vec![vec![1, 2], vec![0], vec![0]];
//! let (tin, tout, order) = euler_tour(&tree, 0);
//! assert_eq!(order[0], 0);
//! assert!(tin[1] < tout[1]);
//! ```

pub fn euler_tour(tree: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = tree.len();
    let mut tin = vec![0; n];
    let mut tout = vec![0; n];
    let mut order = Vec::with_capacity(n);
    let mut timer = 0usize;
    fn dfs(
        node: usize,
        par: usize,
        tree: &[Vec<usize>],
        tin: &mut [usize],
        tout: &mut [usize],
        order: &mut Vec<usize>,
        timer: &mut usize,
    ) {
        tin[node] = *timer;
        *timer += 1;
        order.push(node);
        for &next in &tree[node] {
            if next != par {
                dfs(next, node, tree, tin, tout, order, timer);
            }
        }
        tout[node] = *timer;
    }
    dfs(root, root, tree, &mut tin, &mut tout, &mut order, &mut timer);
    (tin, tout, order)
}

#[cfg(test)]
mod tests {
    use super::euler_tour;

    #[test]
    fn euler_tour_records_entry_exit_and_order() {
        let tree = vec![vec![1, 2], vec![0], vec![0]];
        let (tin, tout, order) = euler_tour(&tree, 0);
        assert_eq!(order[0], 0);
        assert!(tin[1] < tout[1]);
    }
}

