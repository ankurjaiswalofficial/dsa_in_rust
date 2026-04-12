//! Dynamic programming examples.
//!
//! Dynamic programming applies when a problem contains overlapping subproblems and optimal
//! substructure.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::dynamic_programming::{knapsack_01, lis_length};
//!
//! assert_eq!(lis_length(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
//! assert_eq!(knapsack_01(&[(2, 6), (2, 10), (3, 12)], 5), 22);
//! ```

pub fn fibonacci_memoized(n: usize) -> u64 {
    fn dfs(n: usize, memo: &mut [Option<u64>]) -> u64 {
        if let Some(value) = memo[n] {
            return value;
        }
        let value = if n <= 1 {
            n as u64
        } else {
            dfs(n - 1, memo) + dfs(n - 2, memo)
        };
        memo[n] = Some(value);
        value
    }

    let mut memo = vec![None; n + 1];
    dfs(n, &mut memo)
}

pub fn fibonacci_tabulation(n: usize) -> u64 {
    let mut dp = vec![0_u64; n + 1];
    if n >= 1 {
        dp[1] = 1;
    }
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

pub fn coin_change_min(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX / 4; amount + 1];
    dp[0] = 0;
    for current in 1..=amount {
        for &coin in coins {
            if coin <= current {
                dp[current] = dp[current].min(dp[current - coin] + 1);
            }
        }
    }
    (dp[amount] < usize::MAX / 4).then_some(dp[amount])
}

pub fn lis_length(values: &[i64]) -> usize {
    let mut tails = Vec::new();
    for &value in values {
        match tails.binary_search(&value) {
            Ok(index) => tails[index] = value,
            Err(index) => {
                if index == tails.len() {
                    tails.push(value);
                } else {
                    tails[index] = value;
                }
            }
        }
    }
    tails.len()
}

pub fn knapsack_01(items: &[(usize, i64)], capacity: usize) -> i64 {
    let mut dp = vec![0; capacity + 1];
    for &(weight, value) in items {
        for current in (weight..=capacity).rev() {
            dp[current] = dp[current].max(dp[current - weight] + value);
        }
    }
    dp[capacity]
}

pub fn knapsack_unbounded(items: &[(usize, i64)], capacity: usize) -> i64 {
    let mut dp = vec![0; capacity + 1];
    for current in 0..=capacity {
        for &(weight, value) in items {
            if weight <= current {
                dp[current] = dp[current].max(dp[current - weight] + value);
            }
        }
    }
    dp[capacity]
}

pub fn longest_common_subsequence(left: &str, right: &str) -> usize {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let mut dp = vec![vec![0; right.len() + 1]; left.len() + 1];
    for i in 1..=left.len() {
        for j in 1..=right.len() {
            dp[i][j] = if left[i - 1] == right[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[left.len()][right.len()]
}

pub fn matrix_chain_multiplication(dimensions: &[usize]) -> usize {
    let n = dimensions.len().saturating_sub(1);
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0; n]; n];
    for len in 2..=n {
        for left in 0..=n - len {
            let right = left + len - 1;
            dp[left][right] = usize::MAX;
            for split in left..right {
                let cost = dp[left][split]
                    + dp[split + 1][right]
                    + dimensions[left] * dimensions[split + 1] * dimensions[right + 1];
                dp[left][right] = dp[left][right].min(cost);
            }
        }
    }
    dp[0][n - 1]
}

pub fn unique_paths(rows: usize, cols: usize) -> usize {
    let mut dp = vec![vec![0; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            dp[row][col] = if row == 0 || col == 0 {
                1
            } else {
                dp[row - 1][col] + dp[row][col - 1]
            };
        }
    }
    dp[rows - 1][cols - 1]
}

pub fn tree_max_independent_set(tree: &[Vec<usize>], root: usize) -> i32 {
    fn dfs(node: usize, parent: usize, tree: &[Vec<usize>]) -> (i32, i32) {
        let mut include = 1;
        let mut exclude = 0;
        for &next in &tree[node] {
            if next == parent {
                continue;
            }
            let (child_include, child_exclude) = dfs(next, node, tree);
            include += child_exclude;
            exclude += child_include.max(child_exclude);
        }
        (include, exclude)
    }

    let (include, exclude) = dfs(root, usize::MAX, tree);
    include.max(exclude)
}

#[cfg(test)]
mod tests {
    use super::{
        coin_change_min, fibonacci_memoized, fibonacci_tabulation, knapsack_01, knapsack_unbounded,
        lis_length, longest_common_subsequence, matrix_chain_multiplication,
        tree_max_independent_set, unique_paths,
    };

    #[test]
    fn dp_examples_cover_common_patterns() {
        assert_eq!(coin_change_min(&[1, 3, 4], 6), Some(2));
        assert_eq!(coin_change_min(&[5, 7], 1), None);
        assert_eq!(lis_length(&[0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(knapsack_01(&[(1, 1), (3, 4), (4, 5), (5, 7)], 7), 9);
        assert_eq!(fibonacci_memoized(10), 55);
        assert_eq!(fibonacci_tabulation(10), 55);
        assert_eq!(knapsack_unbounded(&[(2, 3), (3, 4), (4, 5)], 7), 10);
        assert_eq!(longest_common_subsequence("abcde", "ace"), 3);
        assert_eq!(matrix_chain_multiplication(&[40, 20, 30, 10, 30]), 26_000);
        assert_eq!(unique_paths(3, 7), 28);
        let tree = vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]];
        assert_eq!(tree_max_independent_set(&tree, 0), 3);
    }
}
