use enum_iterator::IntoEnumIterator;
use instant::Duration;

use crate::{GameState, Move};
use crate::agents::{alphabeta, greedy, minimax, montecarlo};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum AiAgent {
    Greedy,
    PureMonteCarlo,
    HybridMonteCarlo,
    Minimax,
    Alphabeta,
}

impl AiAgent {
    pub fn play_move(&self, state: &GameState, duration: Duration) -> Option<(Move, i64)> {
        match self {
            AiAgent::Greedy => greedy::greedy_agent(state),
            AiAgent::PureMonteCarlo => montecarlo::pure_montecarlo_agent(state, duration),
            AiAgent::HybridMonteCarlo => montecarlo::hybrid_hard_montecarlo_agent(state, duration),
            AiAgent::Minimax => minimax::iterative_deepening(state, duration),
            AiAgent::Alphabeta => alphabeta::iterative_deepening(state, duration),
        }
    }
}
