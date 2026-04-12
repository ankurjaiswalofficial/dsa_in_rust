//! Branch and bound for 0/1 knapsack.
//!
//! Branch and bound improves brute-force search by estimating the best possible score inside a
//! branch. If that upper bound is already worse than the current best answer, the branch is pruned.
//!
//! # Example
//!
//! ```
//! use project1::advanced::backtracking_topics::branch_and_bound::knapsack_branch_and_bound;
//!
//! let items = vec![(2, 40), (3, 50), (5, 100), (4, 60)];
//! assert_eq!(knapsack_branch_and_bound(&items, 8), 150);
//! ```

pub fn knapsack_branch_and_bound(items: &[(i32, i32)], capacity: i32) -> i32 {
    let mut items = items.to_vec();
    items.sort_by(|a, b| (b.1 * a.0).cmp(&(a.1 * b.0)));
    fn bound(items: &[(i32, i32)], index: usize, weight: i32, value: i32, capacity: i32) -> f64 {
        let mut total_weight = weight;
        let mut total_value = value as f64;
        for &(item_weight, item_value) in &items[index..] {
            if total_weight + item_weight <= capacity {
                total_weight += item_weight;
                total_value += item_value as f64;
            } else {
                total_value += (capacity - total_weight) as f64 * item_value as f64 / item_weight as f64;
                break;
            }
        }
        total_value
    }

    fn dfs(items: &[(i32, i32)], index: usize, weight: i32, value: i32, capacity: i32, best: &mut i32) {
        if index == items.len() {
            *best = (*best).max(value);
            return;
        }
        if bound(items, index, weight, value, capacity) <= *best as f64 {
            return;
        }
        let (item_weight, item_value) = items[index];
        if weight + item_weight <= capacity {
            dfs(items, index + 1, weight + item_weight, value + item_value, capacity, best);
        }
        dfs(items, index + 1, weight, value, capacity, best);
    }

    let mut best = 0;
    dfs(&items, 0, 0, 0, capacity, &mut best);
    best
}

#[cfg(test)]
mod tests {
    use super::knapsack_branch_and_bound;

    #[test]
    fn branch_and_bound_prunes_knapsack_search() {
        let items = vec![(2, 40), (3, 50), (5, 100), (4, 60)];
        assert_eq!(knapsack_branch_and_bound(&items, 8), 150);
    }
}
