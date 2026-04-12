//! Floyd-Warshall all-pairs shortest paths.
//!
//! Floyd-Warshall computes the shortest distance between every pair of vertices. The central idea
//! is dynamic programming over the set of allowed intermediate vertices: after processing `k`, the
//! table knows the best paths that may use only vertices `0..=k` as intermediates.
//!
//! It runs in `O(n^3)`, which is practical for dense or small graphs.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::floyd_warshall_algorithm::floyd_warshall;
//!
//! let inf = i64::MAX / 4;
//! let dist = vec![
//!     vec![0, 3, inf, 7],
//!     vec![8, 0, 2, inf],
//!     vec![5, inf, 0, 1],
//!     vec![2, inf, inf, 0],
//! ];
//! let answer = floyd_warshall(dist);
//! assert_eq!(answer[0][2], 5);
//! assert_eq!(answer[3][1], 5);
//! ```

pub fn floyd_warshall(mut dist: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let through = dist[i][k].saturating_add(dist[k][j]);
                if through < dist[i][j] {
                    dist[i][j] = through;
                }
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::floyd_warshall;

    #[test]
    fn floyd_warshall_computes_all_pairs_shortest_paths() {
        let inf = i64::MAX / 4;
        let dist = vec![
            vec![0, 3, inf, 7],
            vec![8, 0, 2, inf],
            vec![5, inf, 0, 1],
            vec![2, inf, inf, 0],
        ];
        let answer = floyd_warshall(dist);
        assert_eq!(answer[0][2], 5);
        assert_eq!(answer[3][1], 5);
    }
}
