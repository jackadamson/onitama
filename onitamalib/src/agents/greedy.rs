use crate::minimax;
use crate::models::{GameState, Move, Player};
use rand::prelude::*;

pub fn greedy_agent(state: &GameState) -> Option<(Move, i64)> {
    let board = match state {
        GameState::Playing { board, .. } => Some(board),
        GameState::Finished { .. } => None,
    }?;
    let mut rng = thread_rng();
    let mut legal_moves = board.legal_moves();
    legal_moves.shuffle(&mut rng);
    let mut legal_moves: Vec<(Move, i64)> = legal_moves
        .into_iter()
        .map(|game_move| {
            let state = board.try_move(game_move).unwrap();
            (game_move, minimax::minimax(&state, 3))
        })
        .collect();
    let key = |(_, score): &(Move, i64)| *score;
    legal_moves.sort_by_key(key);
    match board.turn {
        Player::Red => legal_moves.pop(),
        Player::Blue => Some(*legal_moves.get(0)?),
    }
}
