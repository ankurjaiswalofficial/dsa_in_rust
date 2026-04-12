//! Advanced tree DP with rerooting.
//!
//! Tree DP often starts by computing information for a rooted subtree, then extends to rerooting
//! so every node can be treated as the root in overall `O(n)`.
//!
//! The function here computes the sum of distances from every node to all other nodes.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::tree_dp_advanced::sum_of_distances_in_tree;
//!
//! let tree = vec![vec![1, 2], vec![0], vec![0]];
//! assert_eq!(sum_of_distances_in_tree(&tree), vec![2, 3, 3]);
//! ```

pub fn sum_of_distances_in_tree(tree: &[Vec<usize>]) -> Vec<i32> {
    let n = tree.len();
    let mut size = vec![1_i32; n];
    let mut answer = vec![0_i32; n];

    fn dfs1(node: usize, parent: usize, tree: &[Vec<usize>], size: &mut [i32], answer: &mut [i32]) {
        for &next in &tree[node] {
            if next == parent {
                continue;
            }
            dfs1(next, node, tree, size, answer);
            size[node] += size[next];
            answer[node] += answer[next] + size[next];
        }
    }

    fn dfs2(node: usize, parent: usize, tree: &[Vec<usize>], size: &[i32], answer: &mut [i32], n: i32) {
        for &next in &tree[node] {
            if next == parent {
                continue;
            }
            answer[next] = answer[node] - size[next] + (n - size[next]);
            dfs2(next, node, tree, size, answer, n);
        }
    }

    dfs1(0, usize::MAX, tree, &mut size, &mut answer);
    dfs2(0, usize::MAX, tree, &size, &mut answer, n as i32);
    answer
}

#[cfg(test)]
mod tests {
    use super::sum_of_distances_in_tree;

    #[test]
    fn tree_dp_with_rerooting_computes_distance_sums() {
        let tree = vec![vec![1, 2], vec![0], vec![0]];
        assert_eq!(sum_of_distances_in_tree(&tree), vec![2, 3, 3]);
    }
}
