//! Bitmask DP from a bit-manipulation angle.
//!
//! Bitmask DP uses an integer to represent a subset. That makes state transitions cheap because
//! adding, removing, and checking membership all reduce to bit operations.
//!
//! The example here solves a tiny assignment-style problem: choose one cost from each row while
//! using each column at most once.
//!
//! # Example
//!
//! ```
//! use project1::advanced::bit_manipulation_topics::bitmask_dp::minimum_assignment_cost;
//!
//! let cost = vec![vec![9, 2, 7], vec![6, 4, 3], vec![5, 8, 1]];
//! assert_eq!(minimum_assignment_cost(&cost), 9);
//! ```

pub fn minimum_assignment_cost(cost: &[Vec<i32>]) -> i32 {
    let n = cost.len();
    let size = 1usize << n;
    let mut dp = vec![i32::MAX / 4; size];
    dp[0] = 0;
    for mask in 0..size {
        let worker = mask.count_ones() as usize;
        if worker >= n {
            continue;
        }
        for job in 0..n {
            if (mask >> job) & 1 == 0 {
                let next = mask | (1 << job);
                dp[next] = dp[next].min(dp[mask] + cost[worker][job]);
            }
        }
    }
    dp[size - 1]
}

#[cfg(test)]
mod tests {
    use super::minimum_assignment_cost;

    #[test]
    fn bitmask_dp_solves_small_assignment_instances() {
        let cost = vec![vec![9, 2, 7], vec![6, 4, 3], vec![5, 8, 1]];
        assert_eq!(minimum_assignment_cost(&cost), 9);
    }
}
