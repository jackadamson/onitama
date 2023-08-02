use serde::{Deserialize, Serialize};

use crate::models::Player;
use crate::{GameState, Move};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameMessage {
    Joined,
    RequestRematch,
    Disconnected,
    Initialize {
        state: GameState,
        room_id: String,
        player: Player,
        waiting: bool,
    },
    Move {
        game_move: Move,
    },
    Error {
        message: String,
    },
}
