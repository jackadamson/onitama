use instant::{Duration, Instant};

use crate::models::{GameState, Move, Player};

const MAX_DEPTH: u16 = 50;
pub fn iterative_deepening(state: &GameState, duration: Duration) -> Option<(Move, i64)> {
    let start = Instant::now();
    let deadline = start + duration;
    let mut result: Option<(Move, i64)> = None;
    for depth in 1..MAX_DEPTH {
        match optimal_move_deadline(state, depth, deadline) {
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

fn optimal_move_deadline(state: &GameState, depth: u16, deadline: Instant) -> Option<(Move, i64)> {
    let timedout = || Instant::now() >  deadline;
    let board = match state {
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
    let state = board
        .try_move(best_move)
        .expect("generated illegal move");
    let mut best_score = minimax(&state, depth - 1);
    for game_move in game_moves {
        if timedout() { return None; }
        let state = board
            .try_move(game_move)
            .expect("generated illegal move");
        let expected_score = minimax(&state, depth - 1);
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

pub fn optimal_move(state: &GameState, depth: u16) -> Option<(Move, i64)> {
    let board = match state {
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
    let mut best_move = game_moves.next()?;
    let state = board
        .try_move(best_move)
        .expect("generated illegal move");
    let mut best_score = minimax(&state, depth - 1);
    for game_move in game_moves {
        let state = board
            .try_move(game_move)
            .expect("generated illegal move");
        let expected_score = minimax(&state, depth - 1);
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

pub fn minimax(state: &GameState, depth: u16) -> i64 {
    if depth == 0 {
        return state.basic_value();
    }
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return state.basic_value();
        },
    };
    let expected_scores = board
        .legal_moves()
        .into_iter()
        .map(|game_move| {
            let state =  board
                    .try_move(game_move)
                    .expect("illegal move generated");
            minimax(&state, depth - 1)
        });
    match board.turn {
        Player::Red => expected_scores.max().expect("no expected_scores"),
        Player::Blue => expected_scores.min().expect("no expected_scores"),
    }
}
