//! Backtracking algorithms.
//!
//! Backtracking explores a search tree by making a choice, recursing, and then
//! undoing that choice before trying the next option. It is appropriate when
//! the space of candidates is combinatorial but constraints allow pruning.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::backtracking::{generate_subsets, permutations};
//!
//! assert_eq!(generate_subsets(&[1, 2]).len(), 4);
//! assert_eq!(permutations(&[1, 2, 3]).len(), 6);
//! ```

pub fn generate_subsets(values: &[i32]) -> Vec<Vec<i32>> {
    fn dfs(index: usize, values: &[i32], current: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if index == values.len() {
            answer.push(current.clone());
            return;
        }

        dfs(index + 1, values, current, answer);
        current.push(values[index]);
        dfs(index + 1, values, current, answer);
        current.pop();
    }

    let mut answer = Vec::new();
    let mut current = Vec::new();
    dfs(0, values, &mut current, &mut answer);
    answer
}

pub fn permutations(values: &[i32]) -> Vec<Vec<i32>> {
    fn dfs(values: &[i32], used: &mut [bool], current: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if current.len() == values.len() {
            answer.push(current.clone());
            return;
        }

        for index in 0..values.len() {
            if used[index] {
                continue;
            }
            used[index] = true;
            current.push(values[index]);
            dfs(values, used, current, answer);
            current.pop();
            used[index] = false;
        }
    }

    let mut answer = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; values.len()];
    dfs(values, &mut used, &mut current, &mut answer);
    answer
}

pub fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    fn dfs(
        index: usize,
        target: i32,
        candidates: &[i32],
        current: &mut Vec<i32>,
        answer: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            answer.push(current.clone());
            return;
        }
        if index == candidates.len() || target < 0 {
            return;
        }

        current.push(candidates[index]);
        dfs(index, target - candidates[index], candidates, current, answer);
        current.pop();
        dfs(index + 1, target, candidates, current, answer);
    }

    let mut candidates = candidates.to_vec();
    candidates.sort();
    let mut answer = Vec::new();
    let mut current = Vec::new();
    dfs(0, target, &candidates, &mut current, &mut answer);
    answer
}

pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    fn dfs(
        row: usize,
        n: usize,
        cols: &mut [bool],
        diag1: &mut [bool],
        diag2: &mut [bool],
        board: &mut Vec<usize>,
        answer: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            answer.push(
                board
                    .iter()
                    .map(|&col| {
                        let mut chars = vec!['.'; n];
                        chars[col] = 'Q';
                        chars.into_iter().collect()
                    })
                    .collect(),
            );
            return;
        }

        for col in 0..n {
            let d1 = row + col;
            let d2 = row + (n - 1 - col);
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;
            board.push(col);
            dfs(row + 1, n, cols, diag1, diag2, board, answer);
            board.pop();
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }

    let mut answer = Vec::new();
    let mut board = Vec::new();
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; n * 2];
    let mut diag2 = vec![false; n * 2];
    dfs(0, n, &mut cols, &mut diag1, &mut diag2, &mut board, &mut answer);
    answer
}

pub fn solve_sudoku(board: &mut [[char; 9]; 9]) -> bool {
    fn valid(board: &[[char; 9]; 9], row: usize, col: usize, digit: char) -> bool {
        for i in 0..9 {
            if board[row][i] == digit || board[i][col] == digit {
                return false;
            }
        }
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for r in start_row..start_row + 3 {
            for c in start_col..start_col + 3 {
                if board[r][c] == digit {
                    return false;
                }
            }
        }
        true
    }

    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] != '.' {
                continue;
            }
            for digit in '1'..='9' {
                if valid(board, row, col, digit) {
                    board[row][col] = digit;
                    if solve_sudoku(board) {
                        return true;
                    }
                    board[row][col] = '.';
                }
            }
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{combination_sum, generate_subsets, permutations, solve_n_queens, solve_sudoku};

    #[test]
    fn subsets_cover_all_choices() {
        let mut answer = generate_subsets(&[1, 2]);
        answer.sort();
        assert_eq!(answer, vec![vec![], vec![1], vec![1, 2], vec![2]]);
    }

    #[test]
    fn permutations_generate_all_orders() {
        let answer = permutations(&[1, 2, 3]);
        assert_eq!(answer.len(), 6);
        assert!(answer.contains(&vec![1, 2, 3]));
        assert!(answer.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn more_backtracking_problems_are_covered() {
        let mut combos = combination_sum(&[2, 3, 6, 7], 7);
        combos.sort();
        assert_eq!(combos, vec![vec![2, 2, 3], vec![7]]);

        assert_eq!(solve_n_queens(4).len(), 2);

        let mut board = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(solve_sudoku(&mut board));
        assert_eq!(board[0][2], '4');
    }
}
