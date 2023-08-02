use std::cell::Cell;

use instant::{Duration, Instant};
use rand::prelude::*;

use crate::{alphabeta, Board, GameState, Move, Player};

pub fn hybrid_montecarlo_agent(state: &GameState, duration: Duration) -> Option<(Move, i64)> {
    log::debug!("Game State: {:?}", state);
    let alphabeta_duration = duration / 2;
    let moves_scored = match alphabeta::moves_scored_deepening(state, alphabeta_duration) {
        None => {
            return None;
        }
        Some(val) => val,
    };
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return None;
        }
    };
    let guaranteed_win_score = match board.turn {
        Player::Red => i64::MAX,
        Player::Blue => i64::MIN,
    };
    for (game_move, expected_score) in moves_scored.iter() {
        if *expected_score == guaranteed_win_score {
            log::info!("Guaranteed to win");
            return Some((*game_move, *expected_score));
        }
    }
    log::info!("Using monte-carlo");
    let guaranteed_lose_score = match board.turn.invert() {
        Player::Red => i64::MAX,
        Player::Blue => i64::MIN,
    };
    let moves: Vec<Move> = moves_scored
        .iter()
        .filter_map(
            |(game_move, expected_score)| match *expected_score == guaranteed_lose_score {
                true => {
                    log::debug!("Ruling out move: {:?}", game_move);
                    None
                }
                false => {
                    log::debug!("Acceptable score: {:?} - {:?}", expected_score, game_move);
                    Some(*game_move)
                }
            },
        )
        .collect();
    // If all moves lead to loss, still choose a move
    let moves = match moves.len() > 0 {
        true => moves,
        false => {
            log::debug!("Opponent can force a win");
            moves_scored
                .iter()
                .map(|(game_move, _)| *game_move)
                .collect()
        }
    };
    if moves.len() == 1 {
        log::debug!("One legal move");
        return Some((moves[0], 0));
    }
    let monte_carlo_duration = duration / 2;
    let scored_moves = montecarlo(board, moves, monte_carlo_duration);
    let compare = match board.turn {
        Player::Red => |a, b| a > b,
        Player::Blue => |a, b| a < b,
    };
    scored_moves
        .into_iter()
        .reduce(
            |(move_a, score_a), (move_b, score_b)| match compare(score_a, score_b) {
                true => (move_a, score_a),
                false => (move_b, score_b),
            },
        )
}
pub fn hybrid_hard_montecarlo_agent(state: &GameState, duration: Duration) -> Option<(Move, i64)> {
    log::debug!("Game State: {:?}", state);
    let alphabeta_duration = duration / 2;
    let moves_scored = match alphabeta::moves_scored_deepening(state, alphabeta_duration) {
        None => {
            return None;
        }
        Some(val) => val,
    };
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return None;
        }
    };
    let guaranteed_win_score = match board.turn {
        Player::Red => i64::MAX,
        Player::Blue => i64::MIN,
    };
    for (game_move, expected_score) in moves_scored.iter() {
        if *expected_score == guaranteed_win_score {
            log::info!("Guaranteed to win");
            return Some((*game_move, *expected_score));
        }
    }
    log::info!("Using monte-carlo");
    let compare = match board.turn {
        Player::Red => i64::max,
        Player::Blue => i64::min,
    };
    let best_move = moves_scored
        .iter()
        .map(|(_, expected_score)| *expected_score)
        .reduce(compare)
        .unwrap();
    let moves: Vec<Move> = moves_scored
        .iter()
        .filter_map(
            |(game_move, expected_score)| match *expected_score == best_move {
                false => {
                    log::debug!("Ruling out move: {} - {:?}", expected_score, game_move);
                    None
                }
                true => {
                    log::debug!("Acceptable score: {:?} - {:?}", expected_score, game_move);
                    Some(*game_move)
                }
            },
        )
        .collect();
    // If all moves lead to loss, still choose a move
    let moves = match moves.len() > 0 {
        true => moves,
        false => {
            log::debug!("Opponent can force a win");
            moves_scored
                .iter()
                .map(|(game_move, _)| *game_move)
                .collect()
        }
    };
    if moves.len() == 1 {
        log::debug!("One legal move");
        return Some((moves[0], 0));
    }
    let monte_carlo_duration = duration / 2;
    let scored_moves = montecarlo(board, moves, monte_carlo_duration);
    let compare = match board.turn {
        Player::Red => |a, b| a > b,
        Player::Blue => |a, b| a < b,
    };
    let result = scored_moves
        .into_iter()
        .reduce(
            |(move_a, score_a), (move_b, score_b)| match compare(score_a, score_b) {
                true => (move_a, score_a),
                false => (move_b, score_b),
            },
        );
    if let Some((_, expected_score)) = &result {
        log::debug!("Expected result: {}", expected_score);
    }
    result
}

pub fn hybrid_hard_montecarlo_rank_moves(
    state: &GameState,
    duration: Duration,
) -> Option<Vec<(Move, i64)>> {
    log::debug!("Game State: {:?}", state);
    let alphabeta_duration = duration / 2;
    let alphabeta_scored_moves = match alphabeta::moves_scored_deepening(state, alphabeta_duration)
    {
        None => {
            return None;
        }
        Some(val) => val,
    };
    let board = match state {
        GameState::Playing { board } => board,
        GameState::Finished { .. } => {
            return None;
        }
    };
    let moves: Vec<Move> = alphabeta_scored_moves
        .iter()
        .map(|(game_move, _expected_score)| *game_move)
        .collect();
    if moves.len() == 1 {
        log::debug!("One legal move");
        return Some(alphabeta_scored_moves);
    }
    let monte_carlo_duration = duration / 2;
    let monte_carlo_scored_moves = montecarlo(board, moves, monte_carlo_duration);
    let result: Vec<(Move, i64)> = alphabeta_scored_moves
        .into_iter()
        .zip(monte_carlo_scored_moves.into_iter())
        .map(|((alpha_move, alpha_score), (monte_move, monte_score))| {
            if alpha_move != monte_move {
                panic!("monte move does not match alpha move");
            }
            if alpha_score == i64::MIN || alpha_score == i64::MAX {
                return (alpha_move, alpha_score);
            }
            return (alpha_move, (alpha_score / 2) + (monte_score / 2));
        })
        .collect();
    Some(result)
}

const ITERATIONS_PER_TIME_CHECK: u8 = 50;

fn montecarlo(board: &Board, moves: Vec<Move>, duration: Duration) -> Vec<(Move, i64)> {
    let start = Instant::now();
    let deadline = start + duration;
    let timedout = || Instant::now() > deadline;
    let results: Vec<(Move, Cell<i64>)> = moves
        .into_iter()
        .map(|game_move| (game_move, Cell::new(0i64)))
        .collect();
    let mut simulations = 0u64;
    cfg_if::cfg_if! {
        if #[cfg(feature = "web")] {
            let mut rng = rand_mt::Mt::default();
        } else {
            let mut rng = thread_rng();
        }
    }
    while !timedout() {
        for _ in 0..ITERATIONS_PER_TIME_CHECK {
            for (game_move, score) in results.iter() {
                simulations += 1;
                let state = board.try_move(*game_move).expect("illegal move");
                let new_score = score.get()
                    + match simulate(state, &mut rng) {
                        Some(Player::Red) => 1,
                        Some(Player::Blue) => -1,
                        None => 0,
                    };
                score.set(new_score);
            }
        }
    }
    log::info!("Monte-carlo timed out after {} simulations", simulations);
    return results
        .into_iter()
        .map(|(game_move, score)| (game_move, score.get()))
        .collect();
}

#[cfg(test)]
pub fn montecarlo_count_simulations(board: &Board, moves: Vec<Move>, duration: Duration) -> u64 {
    let start = Instant::now();
    let deadline = start + duration;
    let timedout = || Instant::now() > deadline;
    let results: Vec<(Move, Cell<i64>)> = moves
        .into_iter()
        .map(|game_move| (game_move, Cell::new(0i64)))
        .collect();
    let mut simulations = 0u64;
    cfg_if::cfg_if! {
        if #[cfg(feature = "web")] {
            let mut rng = rand_mt::Mt::default();
        } else {
            let mut rng = thread_rng();
        }
    }
    while !timedout() {
        for _ in 0..ITERATIONS_PER_TIME_CHECK {
            for (game_move, score) in results.iter() {
                simulations += 1;
                let state = board.try_move(*game_move).expect("illegal move");
                let new_score = score.get()
                    + match simulate(state, &mut rng) {
                        Some(Player::Red) => 1,
                        Some(Player::Blue) => -1,
                        None => 0,
                    };
                score.set(new_score);
            }
        }
    }
    log::info!("Monte-carlo timed out after {} simulations", simulations);
    return simulations;
}

pub fn pure_montecarlo_agent(state: &GameState, duration: Duration) -> Option<(Move, i64)> {
    let board = match state {
        GameState::Playing { board, .. } => Some(*board),
        GameState::Finished { .. } => None,
    }?;
    let moves = board.legal_moves();
    let scored_moves = montecarlo(&board, moves, duration);
    let compare = match board.turn {
        Player::Red => |a, b| a > b,
        Player::Blue => |a, b| a < b,
    };
    scored_moves
        .into_iter()
        .reduce(
            |(move_a, score_a), (move_b, score_b)| match compare(score_a, score_b) {
                true => (move_a, score_a),
                false => (move_b, score_b),
            },
        )
}

// Choose random moves and return the player that one, or None if loop
fn simulate<R: Rng>(state: GameState, rng: &mut R) -> Option<Player> {
    let mut state = state;
    for _ in 0..1000 {
        let board = match state {
            GameState::Playing { board } => board,
            GameState::Finished { winner, .. } => {
                return Some(winner);
            }
        };
        let game_move = board.random_legal_move(rng);
        state = state
            .try_move(game_move)
            .expect("montecarlo played illegal move");
    }
    None
}
