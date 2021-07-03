use std::cmp;

use instant::{Duration, Instant};

use crate::models::{GameState, Move, Player};

const MAX_DEPTH: u16 = 50;

pub fn iterative_deepening(state: &GameState, duration: Duration) -> Option<(Move, i8)> {
    let start = Instant::now();
    let deadline = start + duration;
    let mut result: Option<(Move, i8)> = None;
    for depth in 1..MAX_DEPTH {
        if let Some((_, val)) = result {
            if val == i8::MAX || val == i8::MIN {
                break;
            }
        }
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

fn optimal_move_deadline(state: &GameState, depth: u16, deadline: Instant) -> Option<(Move, i8)> {
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
    let mut best_score = minimax(&state,depth - 1, i8::MIN, i8::MAX);
    for game_move in game_moves {
        if timedout() { return None; }
        let state = board.try_move(game_move)
            .expect("generated illegal move in loop");
        let expected_score = minimax(&state, depth - 1, i8::MIN, i8::MAX);
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

pub fn moves_scored_deepening(state: &GameState, duration: Duration) -> Option<Vec<(Move, i8)>> {
    let start = Instant::now();
    let deadline = start + duration;
    let mut result: Option<Vec<(Move, i8)>> = None;
    for depth in 1..MAX_DEPTH {
        match moves_scored_deadline(state, depth, deadline) {
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

fn moves_scored_deadline(state: &GameState, depth: u16, deadline: Instant) -> Option<Vec<(Move, i8)>> {
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
    let mut scored_moves: Vec<(Move, i8)> = vec![];
    let game_moves = board
        .legal_moves()
        .into_iter();
    for game_move in game_moves {
        if timedout() { return None; }
        let state = board.try_move(game_move)
            .expect("generated illegal move in loop");
        let expected_score = minimax(&state, depth - 1, i8::MIN, i8::MAX);
        scored_moves.push((game_move, expected_score));
    }
    return Some(scored_moves);
}

fn minimax(state: &GameState, depth: u16, mut alpha: i8, mut beta: i8) -> i8 {
    if depth == 0 {
        return state.basic_value();
    }
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return state.basic_value();
        },
    };
    let mut value = match board.turn {
        Player::Red => i8::MIN,
        Player::Blue => i8::MAX,
    };
    let legal_moves = board.legal_moves().into_iter();
    for game_move in legal_moves {
        let state = board
            .try_move(game_move)
            .expect("illegal move generated");
        let next_val = minimax(&state, depth - 1, alpha, beta);
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

pub fn optimal_move(state: &GameState, depth: u16) -> Option<(Move, i8)> {
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
    let mut best_move = game_moves
        .next()
        .expect("No legal moves");
    let state = board
        .try_move(best_move)
        .expect("generated illegal move");
    let mut best_score = minimax(&state, depth - 1, i8::MIN, i8::MAX);
    for game_move in game_moves {
        let state = board.try_move(game_move)
            .expect("generated illegal move in loop");
        let expected_score = minimax(&state, depth - 1, i8::MIN, i8::MAX);
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
