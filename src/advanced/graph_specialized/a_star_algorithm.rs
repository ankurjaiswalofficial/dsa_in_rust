//! A* algorithm.
//!
//! A* uses a heuristic to guide shortest-path search toward the goal. If the heuristic never
//! overestimates the remaining cost, A* stays optimal.
//!
//! The implementation below searches a grid with Manhattan-distance heuristic.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graph_specialized::a_star_algorithm::shortest_path_on_grid;
//!
//! let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]];
//! assert_eq!(shortest_path_on_grid(&grid, (0, 0), (2, 2)), Some(4));
//! ```

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn shortest_path_on_grid(grid: &[Vec<i32>], start: (usize, usize), goal: (usize, usize)) -> Option<i32> {
    let rows = grid.len();
    let cols = grid[0].len();
    let heuristic = |(r, c): (usize, usize)| -> i32 {
        (goal.0.abs_diff(r) + goal.1.abs_diff(c)) as i32
    };
    let mut dist = vec![vec![i32::MAX / 4; cols]; rows];
    let mut heap = BinaryHeap::new();
    dist[start.0][start.1] = 0;
    heap.push((Reverse(heuristic(start)), 0_i32, start));
    while let Some((Reverse(_score), cost, (r, c))) = heap.pop() {
        if (r, c) == goal {
            return Some(cost);
        }
        if cost != dist[r][c] {
            continue;
        }
        for (nr, nc) in [
            (r.wrapping_add(1), c),
            (r.wrapping_sub(1), c),
            (r, c.wrapping_add(1)),
            (r, c.wrapping_sub(1)),
        ] {
            if nr >= rows || nc >= cols || grid[nr][nc] == 1 {
                continue;
            }
            let next_cost = cost + 1;
            if next_cost < dist[nr][nc] {
                dist[nr][nc] = next_cost;
                heap.push((Reverse(next_cost + heuristic((nr, nc))), next_cost, (nr, nc)));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::shortest_path_on_grid;

    #[test]
    fn a_star_finds_a_shortest_grid_path() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]];
        assert_eq!(shortest_path_on_grid(&grid, (0, 0), (2, 2)), Some(4));
    }
}
