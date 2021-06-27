use serde::{Deserialize, Serialize};

use crate::{GameState, Move};
use crate::models::Player;

#[derive(Serialize, Deserialize, Debug)]
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
    }
}
