//! Nim game.
//!
//! Nim is the canonical impartial combinatorial game. The winning condition is determined entirely
//! by the XOR of pile sizes.
//!
//! # Example
//!
//! ```
//! use project1::advanced::game_theory::nim_game::first_player_wins;
//!
//! assert!(first_player_wins(&[1, 4, 6]));
//! assert!(!first_player_wins(&[1, 2, 3]));
//! ```

pub fn first_player_wins(piles: &[u32]) -> bool {
    piles.iter().fold(0, |acc, &pile| acc ^ pile) != 0
}

#[cfg(test)]
mod tests {
    use super::first_player_wins;

    #[test]
    fn nim_winner_is_determined_by_xor() {
        assert!(first_player_wins(&[1, 4, 6]));
        assert!(!first_player_wins(&[1, 2, 3]));
    }
}
