use instant::{Duration, Instant};
use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::{alphabeta, Board, GameState, Move, Player};
use std::cell::Cell;

pub fn montecarlo_agent(state: &GameState, duration: Duration) -> Option<(Move, i64)> {
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
        Player::Red => i8::MAX,
        Player::Blue => i8::MIN,
    };
    for (game_move, expected_score) in moves_scored.iter() {
        if *expected_score == guaranteed_win_score {
            log::info!("Guaranteed to win");
            return Some((*game_move, *expected_score as i64));
        }
    }
    log::info!("Using monte-carlo");
    let guaranteed_lose_score = match board.turn.invert() {
        Player::Red => i8::MAX,
        Player::Blue => i8::MIN,
    };
    let moves: Vec<Move> = moves_scored
        .iter()
        .filter_map(|(game_move, expected_score)| match *expected_score == guaranteed_lose_score {
            true => None,
            false => Some(*game_move),
        })
        .collect();
    // If all moves lead to loss, still choose a move
    let moves = match moves.len() > 0 {
        true => moves,
        false => moves_scored
            .iter()
            .map(|(game_move, _)| *game_move)
            .collect()
    };
    let monte_carlo_duration = duration / 2;
    let scored_moves = montecarlo(board, moves, monte_carlo_duration);
    scored_moves.into_iter().reduce(|(move_a, score_a), (move_b, score_b)| match score_a > score_b {
        true => (move_a, score_a),
        false => (move_b, score_b),
    } )
}

const ITERATIONS_PER_TIME_CHECK: u8 = 50;

fn montecarlo(board: &Board, moves: Vec<Move>, duration: Duration) -> Vec<(Move, i64)> {
    let start = Instant::now();
    let deadline = start + duration;
    let timedout = || Instant::now() >  deadline;
    let results: Vec<(Move, Cell<i64>)> = moves
        .into_iter()
        .map(|game_move| (game_move, Cell::new(0i64)))
        .collect();
    let mut simulations = 0u64;
    while !timedout() {
        for _ in 0..ITERATIONS_PER_TIME_CHECK {
            for (game_move, score) in results.iter() {
                simulations += 1;
                let state = board.try_move(*game_move).expect("illegal move");
                let new_score = score.get() + match simulate(state) {
                    Some(Player::Red) => 1,
                    Some(Player::Blue) => -1,
                    None => 0,
                };
                score.set(new_score);
            }
        }
    }
    log::info!("Monte-carlo timed out after {} simulations", simulations);
    return results.into_iter().map(|(game_move, score)| (game_move, score.get())).collect();
}

// Choose random moves and return the player that one, or None if loop
fn simulate(state: GameState) -> Option<Player> {
    let mut state = state;
    let mut rng = thread_rng();
    for _ in 0..100 {
        let board = match state {
            GameState::Playing { board } => board,
            GameState::Finished { winner, .. } => {
                return Some(winner);
            }
        };
        let moves = board.legal_moves();
        let game_move = moves.into_iter().choose(&mut rng).expect("No legal moves in montecarlo sim");
        state = state.try_move(game_move).expect("montecarlo played illegal move");
    }
    log::info!("Game failed to end after being simulated for 100 turns");
    None
}
