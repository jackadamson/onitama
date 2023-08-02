use super::utils;
use crate::{montecarlo, GameState};
use indicatif::ProgressBar;
use instant::Duration;

#[test]
fn montecarlo_benchmark() {
    let duration = Duration::from_millis(100);
    let bar = ProgressBar::new(100);
    let test_states = utils::generate_test_states();
    let mut total_simulations = 0u64;
    for state in test_states.into_iter() {
        bar.inc(1);
        let board = match state {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => panic!("Unexpected finished state"),
        };
        let moves = board.legal_moves();
        total_simulations += montecarlo::montecarlo_count_simulations(&board, moves, duration);
    }
    bar.finish();
    println!("Total Simulations: {}", total_simulations);
}
