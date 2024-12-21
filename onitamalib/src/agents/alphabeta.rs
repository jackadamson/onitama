use std::cmp;

use instant::{Duration, Instant};

use crate::models::{GameState, Move, Player, Point};
use crate::agents::ninja_logic;

const MAX_DEPTH: u16 = 50;

pub fn iterative_deepening(state: &GameState, duration: Duration) -> Option<(Move, i64)> {
    let start = Instant::now();
    let deadline = start + duration;
    let mut result: Option<(Move, i64)> = None;
    for depth in 1..MAX_DEPTH {
        if let Some((_, val)) = result {
            if val == i64::MAX || val == i64::MIN {
                break;
            }
        }
        match optimal_move_deadline(state, depth, deadline) {
            None => {
                log::info!(
                    "Timeout at depth {}, took {}ms",
                    depth,
                    start.elapsed().as_millis()
                );
                break;
            }
            Some(val) => {
                result = Some(val);
            }
        };
    }
    return result;
}

#[cfg(test)]
pub fn iterative_deepening_just_depth(state: &GameState, duration: Duration) -> Option<u16> {
    let start = Instant::now();
    let deadline = start + duration;
    let mut result: Option<(Move, i64)> = None;
    for depth in 1..MAX_DEPTH {
        if let Some((_, val)) = result {
            if val == i64::MAX || val == i64::MIN {
                break;
            }
        }
        match optimal_move_deadline(state, depth, deadline) {
            None => {
                log::info!(
                    "Timeout at depth {}, took {}ms",
                    depth,
                    start.elapsed().as_millis()
                );
                return Some(depth);
            }
            Some(val) => {
                result = Some(val);
            }
        };
    }
    return None;
}

fn optimal_move_deadline(state: &GameState, depth: u16, deadline: Instant) -> Option<(Move, i64)> {
    let timedout = || Instant::now() > deadline;
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return None;
        }
    };
    if depth == 0 {
        return None;
    }
    if timedout() {
        return None;
    }
    let mut game_moves = board.legal_moves().into_iter();
    let mut best_move = game_moves.next().expect("No legal moves");
    let state = board.try_move(best_move).expect("generated illegal move");
    let mut best_score = minimax(&state, depth - 1, i64::MIN, i64::MAX);
    for game_move in game_moves {
        if timedout() {
            return None;
        }
        let state = board
            .try_move(game_move)
            .expect("generated illegal move in loop");
        let expected_score = minimax(&state, depth - 1, i64::MIN, i64::MAX);
        match board.turn {
            Player::Red if expected_score > best_score => {
                best_move = game_move;
                best_score = expected_score;
            }
            Player::Blue if expected_score < best_score => {
                best_move = game_move;
                best_score = expected_score;
            }
            _ => {}
        };
    }
    return Some((best_move, best_score));
}

pub fn moves_scored_deepening(state: &GameState, duration: Duration) -> Option<Vec<(Move, i64)>> {
    let start = Instant::now();
    let deadline = start + duration;
    let mut result: Option<Vec<(Move, i64)>> = None;
    for depth in 1..MAX_DEPTH {
        match moves_scored_deadline(state, depth, deadline) {
            None => {
                log::info!(
                    "Timeout at depth {}, took {}ms",
                    depth,
                    start.elapsed().as_millis()
                );
                break;
            }
            Some(val) => {
                result = Some(val);
            }
        };
    }
    return result;
}

fn moves_scored_deadline(
    state: &GameState,
    depth: u16,
    deadline: Instant,
) -> Option<Vec<(Move, i64)>> {
    let timedout = || Instant::now() > deadline;
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return None;
        }
    };
    if depth == 0 {
        return None;
    }
    if timedout() {
        return None;
    }
    let mut scored_moves: Vec<(Move, i64)> = vec![];
    let game_moves = board.legal_moves().into_iter();
    for game_move in game_moves {
        if timedout() {
            return None;
        }
        let state = board
            .try_move(game_move)
            .expect("generated illegal move in loop");
        let expected_score = minimax(&state, depth - 1, i64::MIN, i64::MAX);
        scored_moves.push((game_move, expected_score));
    }
    return Some(scored_moves);
}

fn minimax(state: &GameState, depth: u16, mut alpha: i64, mut beta: i64) -> i64 {
    if depth == 0 {
        return state.basic_value();
    }
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return state.basic_value();
        }
    };
    // Clone and randomize hidden Ninja positions
    let mut randomized_state = ninja_logic::randomize_hidden_ninjas(state.clone());

    // Generate possible hidden Ninja positions for the opponent
    let opponent_hidden_ninjas: Vec<Point> = board
        .opponent_ninjas()
        .iter()
        .filter_map(|ninja| ninja.as_ref().filter(|(_, revealed)| !*revealed).map(|(pos, _)| *pos))
        .collect();

    let potential_positions = ninja_logic::possible_ninja_positions(&board, opponent_hidden_ninjas);

    // Replace Ninja positions in the randomized state
    ninja_logic::randomize_ninjas_with_positions(&mut randomized_state, &potential_positions);

    // Extract the updated board from the randomized state
    let updated_board = match &randomized_state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return randomized_state.basic_value();
        }
    };

    let mut value = match updated_board.turn {
        Player::Red => i64::MIN,
        Player::Blue => i64::MAX,
    };

    let legal_moves = updated_board.legal_moves().into_iter();
    for game_move in legal_moves {
        let next_state = updated_board.try_move(game_move).expect("illegal move generated");
        let next_val = minimax(&next_state, depth - 1, alpha, beta);
        value = match updated_board.turn {
            Player::Red => cmp::max(value, next_val),
            Player::Blue => cmp::min(value, next_val),
        };
        match updated_board.turn {
            Player::Red if value >= beta => {
                break;
            }
            Player::Blue if value <= alpha => {
                break;
            }
            Player::Red => {
                alpha = cmp::max(alpha, value);
            }
            Player::Blue => {
                beta = cmp::min(beta, value);
            }
        };
    }
    return value;
}

pub fn optimal_move(state: &GameState, depth: u16) -> Option<(Move, i64)> {
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return None;
        }
    };
    if depth == 0 {
        return None;
    }
    let mut game_moves = board.legal_moves().into_iter();
    let mut best_move = game_moves.next().expect("No legal moves");
    let state = board.try_move(best_move).expect("generated illegal move");
    let mut best_score = minimax(&state, depth - 1, i64::MIN, i64::MAX);
    for game_move in game_moves {
        let state = board
            .try_move(game_move)
            .expect("generated illegal move in loop");
        let expected_score = minimax(&state, depth - 1, i64::MIN, i64::MAX);
        match board.turn {
            Player::Red if expected_score > best_score => {
                best_move = game_move;
                best_score = expected_score;
            }
            Player::Blue if expected_score < best_score => {
                best_move = game_move;
                best_score = expected_score;
            }
            _ => {}
        };
    }
    return Some((best_move, best_score));
}
