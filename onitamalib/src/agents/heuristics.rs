// Red is maximizing player, Blue is minimizing
use crate::models::{Player, GameState};
fn value_from_pawn_count(count: usize) -> i8 {
    // Reserve one point for possibly distinguishing current player
    // Consider pawns more valuable, the fewer there are
    match count {
        0 => 0,
        1 => 8,
        2 => 8 + 6,
        3 => 8 + 6 + 4,
        4 => 8 + 6 + 4 + 2,
        _ => panic!("Invalid number of pawns"),
    }
}
impl GameState {
    pub fn basic_value(&self) -> i8 {
        let board = match self {
            GameState::Playing { board } => board,
            GameState::Finished { winner: Player::Blue, .. } => {
                return i8::MIN;
            },
            GameState::Finished { winner: Player::Red, .. } => {
                return i8::MAX;
            },
        };
        let red_count = board.red_pawns.len();
        let blue_count = board.blue_pawns.len();
        return value_from_pawn_count(red_count) - value_from_pawn_count(blue_count);
    }
}
