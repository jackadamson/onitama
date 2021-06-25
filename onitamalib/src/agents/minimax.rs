use std::time::{Instant, Duration};

use crate::models::{Move, Player, GameState};

const MAX_DEPTH: u16 = 50;

impl GameState {
    pub fn iterative_deepening(&self, duration: Duration) -> Option<(Move, i8)> {
        let deadline = Instant::now() + duration;
        let mut result: Option<(Move, i8)> = None;
        for depth in 1..MAX_DEPTH {
            match self.optimal_move_deadline(depth, deadline) {
                None => {
                    log::info!("Timeout at depth {}", depth);
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
            .minimax(depth - 1);
        for game_move in game_moves {
            if timedout() { return None; }
            let expected_score = board.try_move(game_move)
                .expect("generated illegal move in loop")
                .minimax(depth - 1);
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
            .minimax(depth - 1);
        for game_move in game_moves {
            let expected_score = board.try_move(game_move)
                .expect("generated illegal move in loop")
                .minimax(depth - 1);
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
    fn minimax(&self, depth: u16) -> i8 {
        if depth == 0 {
            return self.basic_value();
        }
        let board = match self {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return self.basic_value();
            },
        };
        let expected_scores = board
            .legal_moves()
            .into_iter()
            .map(|game_move| {
                board
                    .try_move(game_move)
                    .expect("illegal move generated")
                    .minimax(depth - 1)
            });
        match board.turn {
            Player::Red => expected_scores.max().expect("no expected_scores"),
            Player::Blue => expected_scores.min().expect("no expected_scores"),
        }
    }
}
