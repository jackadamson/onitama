use enum_iterator::IntoEnumIterator;
use instant::Duration;
use serde::{Serialize, Deserialize};

use crate::{GameState, Move};
use crate::agents::{alphabeta, greedy, minimax, montecarlo};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
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

cfg_if::cfg_if! {
    if #[cfg(feature = "web")] {
        use wasm_bindgen::prelude::*;
        use crate::MoveRequest;

        #[wasm_bindgen(js_name = agentMove)]
        pub fn agent_move(request: &JsValue) -> JsValue {
            let MoveRequest { state, agent } = request.into_serde().unwrap();
            let duration = Duration::from_millis(1000);
            let (game_move, _) = agent.play_move(&state, duration).unwrap();
            JsValue::from_serde(&game_move).unwrap()
        }

        #[wasm_bindgen(js_name = rankMoves)]
        pub fn rank_moves(request: &JsValue) -> JsValue {
            // Used by training mode to request scoring of all possible moves
            let state: GameState = request.into_serde().unwrap();
            let duration = Duration::from_millis(1000);
            let ranked_moves = montecarlo::hybrid_hard_montecarlo_rank_moves(&state, duration);
            JsValue::from_serde(&ranked_moves).unwrap()
        }
   }
}
