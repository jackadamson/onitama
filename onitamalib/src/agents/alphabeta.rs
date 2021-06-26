use instant::{Instant, Duration};
use std::cmp;

use crate::models::{Move, Player, GameState};

const MAX_DEPTH: u16 = 50;

impl GameState {
    pub fn iterative_deepening(&self, duration: Duration) -> Option<(Move, i8)> {
        let start = Instant::now();
        let deadline = start + duration;
        let mut result: Option<(Move, i8)> = None;
        for depth in 1..MAX_DEPTH {
            match self.optimal_move_deadline(depth, deadline) {
                None => {
                    log::info!("Timeout at depth {}, took {}ms", depth, start.elapsed().as_millis());
                    break;
                }
                Some(val) => {
                    result = Some(val);
                },
            };
        }
        return result;
    }
    pub fn optimal_move_deadline(&self, depth: u16, deadline: Instant) -> Option<(Move, i8)> {
        let timedout = || Instant::now() >  deadline;
        let board = match self {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return None;
            },
        };
        if depth == 0 {
            return None;
        }
        if timedout() { return None; }
        let mut game_moves = board
            .legal_moves()
            .into_iter();
        let mut best_move = game_moves
            .next()
            .expect("No legal moves");
        let mut best_score = board
            .try_move(best_move)
            .expect("generated illegal move")
            .minimax(depth - 1, i8::MIN, i8::MAX);
        for game_move in game_moves {
            if timedout() { return None; }
            let expected_score = board.try_move(game_move)
                .expect("generated illegal move in loop")
                .minimax(depth - 1, i8::MIN, i8::MAX);
            match board.turn {
                Player::Red if expected_score > best_score => {
                    best_move = game_move;
                    best_score = expected_score;
                },
                Player::Blue if expected_score < best_score  => {
                    best_move = game_move;
                    best_score = expected_score;
                },
                _ => {},
            };
        }
        return Some((best_move, best_score));
    }
    pub fn optimal_move(&self, depth: u16) -> Option<(Move, i8)> {
        let board = match self {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return None;
            },
        };
        if depth == 0 {
            return None;
        }
        let mut game_moves = board
            .legal_moves()
            .into_iter();
        let mut best_move = game_moves
            .next()
            .expect("No legal moves");
        let mut best_score = board
            .try_move(best_move)
            .expect("generated illegal move")
            .minimax(depth - 1, i8::MIN, i8::MAX);
        for game_move in game_moves {
            let expected_score = board.try_move(game_move)
                .expect("generated illegal move in loop")
                .minimax(depth - 1, i8::MIN, i8::MAX);
            match board.turn {
                Player::Red if expected_score > best_score => {
                    best_move = game_move;
                    best_score = expected_score;
                },
                Player::Blue if expected_score < best_score  => {
                    best_move = game_move;
                    best_score = expected_score;
                },
                _ => {},
            };
        }
        return Some((best_move, best_score));
    }
    fn minimax(&self, depth: u16, mut alpha: i8, mut beta: i8) -> i8 {
        if depth == 0 {
            return self.basic_value();
        }
        let board = match self {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return self.basic_value();
            },
        };
        let mut value = match board.turn {
            Player::Red => i8::MIN,
            Player::Blue => i8::MAX,
        };
        let legal_moves = board.legal_moves().into_iter();
        for game_move in legal_moves {
            let next_val = board
                .try_move(game_move)
                .expect("illegal move generated")
                .minimax(depth - 1, alpha, beta);
            value = match board.turn {
                Player::Red => cmp::max(value, next_val),
                Player::Blue => cmp::min(value, next_val),
            };
            match board.turn {
                Player::Red if value >= beta => { break; },
                Player::Blue if value <= alpha => { break; },
                Player::Red => {
                    alpha = cmp::max(alpha, value);
                },
                Player::Blue => {
                    beta = cmp::min(beta, value);
                },
            };
        }
        return value;
    }
}
