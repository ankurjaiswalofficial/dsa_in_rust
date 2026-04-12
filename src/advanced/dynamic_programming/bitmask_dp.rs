//! Bitmask DP.
//!
//! Bitmask DP is the standard tool when the state is a subset of up to about 20 items. The
//! example below solves the travelling-salesperson problem for small graphs.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::bitmask_dp::tsp_min_cost;
//!
//! let graph = vec![
//!     vec![0, 10, 15, 20],
//!     vec![10, 0, 35, 25],
//!     vec![15, 35, 0, 30],
//!     vec![20, 25, 30, 0],
//! ];
//! assert_eq!(tsp_min_cost(&graph), 80);
//! ```

pub fn tsp_min_cost(graph: &[Vec<i32>]) -> i32 {
    let n = graph.len();
    let size = 1usize << n;
    let mut dp = vec![vec![i32::MAX / 4; n]; size];
    dp[1][0] = 0;
    for mask in 0..size {
        for u in 0..n {
            if dp[mask][u] >= i32::MAX / 8 {
                continue;
            }
            for v in 0..n {
                if (mask >> v) & 1 == 0 {
                    let next = mask | (1 << v);
                    dp[next][v] = dp[next][v].min(dp[mask][u] + graph[u][v]);
                }
            }
        }
    }
    (0..n)
        .map(|u| dp[size - 1][u] + graph[u][0])
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::tsp_min_cost;

    #[test]
    fn bitmask_dp_solves_small_tsp_instances() {
        let graph = vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0],
        ];
        assert_eq!(tsp_min_cost(&graph), 80);
    }
}
