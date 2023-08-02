use indicatif::ProgressBar;
use instant::Duration;

use crate::{alphabeta, minimax, GameState};

use super::utils;

const SHARED_DEPTH: u16 = 3;

#[test]
fn alphabeta_equivalent_to_minimax() {
    let test_states = utils::generate_test_states();
    let bar = ProgressBar::new(100);
    for state in test_states.into_iter() {
        bar.inc(1);
        let ab_value = match alphabeta::optimal_move(&state, SHARED_DEPTH) {
            None => panic!("Alphabeta returned None"),
            Some((_, expected)) => expected,
        };
        let minimax_value = match minimax::optimal_move(&state, SHARED_DEPTH) {
            None => panic!("Minimax returned None"),
            Some((_, expected)) => expected,
        };
        assert_eq!(ab_value, minimax_value);
    }
    bar.finish();
}

#[test]
fn alphabeta_benchmark() {
    let duration = Duration::from_millis(100);
    let bar = ProgressBar::new(100);
    let test_states = utils::generate_test_states();
    let mut total_depth = 0u64;
    let mut counted = 0u64;
    for state in test_states.into_iter() {
        bar.inc(1);
        match alphabeta::iterative_deepening_just_depth(&state, duration) {
            None => {}
            Some(depth) => {
                total_depth += depth as u64;
                counted += 1;
            }
        };
    }
    bar.finish();
    let average_depth: f64 = total_depth as f64 / counted as f64;
    println!(
        "Average Depth: {} out of {} timeouts",
        average_depth, counted
    );
}
