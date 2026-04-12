//! Advanced constraint solving with Sudoku.
//!
//! Constraint-solving backtracking works best when every choice is followed by immediate
//! propagation. Sudoku is a compact example because every placement constrains a row, column, and
//! subgrid.
//!
//! # Example
//!
//! ```
//! use project1::advanced::backtracking_topics::advanced_constraint_solving::solve_sudoku;
//!
//! let mut board = [
//!     ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
//!     ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
//!     ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
//!     ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
//!     ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
//!     ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
//!     ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
//!     ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
//!     ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
//! ];
//! assert!(solve_sudoku(&mut board));
//! assert_eq!(board[0][2], '4');
//! ```

pub fn solve_sudoku(board: &mut [[char; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == '.' {
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
    }
    true
}

fn valid(board: &[[char; 9]; 9], row: usize, col: usize, digit: char) -> bool {
    for index in 0..9 {
        if board[row][index] == digit || board[index][col] == digit {
            return false;
        }
    }
    let box_row = row / 3 * 3;
    let box_col = col / 3 * 3;
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if board[r][c] == digit {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::solve_sudoku;

    #[test]
    fn advanced_constraint_solver_solves_a_sudoku() {
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
