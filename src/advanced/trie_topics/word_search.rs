//! Word-search backtracking on a 2D board.
//!
//! Word-search problems often combine tries and DFS. For a single word, plain backtracking is
//! enough: try every board cell as a start, move in four directions, and avoid revisiting cells in
//! the current path.
//!
//! # Example
//!
//! ```
//! use project1::advanced::trie_topics::word_search::exists;
//!
//! let board = vec![
//!     vec!['A', 'B', 'C', 'E'],
//!     vec!['S', 'F', 'C', 'S'],
//!     vec!['A', 'D', 'E', 'E'],
//! ];
//! assert!(exists(&board, "ABCCED"));
//! assert!(!exists(&board, "ABCB"));
//! ```

pub fn exists(board: &[Vec<char>], word: &str) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return word.is_empty();
    }
    let target: Vec<char> = word.chars().collect();
    let rows = board.len();
    let cols = board[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    fn dfs(
        board: &[Vec<char>],
        target: &[char],
        visited: &mut [Vec<bool>],
        row: isize,
        col: isize,
        index: usize,
    ) -> bool {
        if index == target.len() {
            return true;
        }
        if row < 0
            || col < 0
            || row as usize >= board.len()
            || col as usize >= board[0].len()
            || visited[row as usize][col as usize]
            || board[row as usize][col as usize] != target[index]
        {
            return false;
        }

        visited[row as usize][col as usize] = true;
        let found = dfs(board, target, visited, row + 1, col, index + 1)
            || dfs(board, target, visited, row - 1, col, index + 1)
            || dfs(board, target, visited, row, col + 1, index + 1)
            || dfs(board, target, visited, row, col - 1, index + 1);
        visited[row as usize][col as usize] = false;
        found
    }

    for row in 0..rows {
        for col in 0..cols {
            if dfs(board, &target, &mut visited, row as isize, col as isize, 0) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::exists;

    #[test]
    fn word_search_finds_valid_words() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exists(&board, "ABCCED"));
        assert!(exists(&board, "SEE"));
    }

    #[test]
    fn word_search_rejects_invalid_paths() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(!exists(&board, "ABCB"));
    }
}
