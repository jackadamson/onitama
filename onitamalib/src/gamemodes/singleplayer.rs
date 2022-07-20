use rand::prelude::*;
use serde::Serialize;
use serde_cbor::ser;
use wasm_bindgen::prelude::*;

use crate::{AiAgent, GameEvent, GameView, MoveRequest, Player};
use crate::gamemodes::base::Game;
use crate::models::Move;

struct PreviousState {
    game: Game,
    last_move: Option<Move>,
}

#[wasm_bindgen]
pub struct SinglePlayerGame {
    game: Game,
    player: Player,
    agent: AiAgent,
    last_move: Option<Move>,
    previous_states: Vec<PreviousState>,
    training_mode: bool,
    on_send_view: js_sys::Function,
    on_send_error: js_sys::Function,
    on_send_event: js_sys::Function,
    request_ai_move: js_sys::Function,
    request_trainer_ranking: js_sys::Function,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SinglePlayerView {
    player: Player,
    #[serde(flatten)]
    game: GameView,
    last_move: Option<Move>,
    can_undo: bool,
}

#[wasm_bindgen]
impl SinglePlayerGame {
    #[wasm_bindgen(constructor)]
    pub fn new(
        difficulty: &str, training_mode: bool, on_send_view: js_sys::Function, on_send_error: js_sys::Function, request_ai_move: js_sys::Function, request_trainer_ranking: js_sys::Function, on_send_event: js_sys::Function,
    ) -> SinglePlayerGame {
        let is_red: bool = random();
        let agent = match difficulty {
            "easy" => AiAgent::PureMonteCarlo,
            "medium" => AiAgent::Alphabeta,
            "hard" => AiAgent::HybridMonteCarlo,
            _ => AiAgent::Alphabeta,
        };
        let player = match is_red {
            true => Player::Red,
            false => Player::Blue,
        };
        let game = Game::new();
        let mut game = SinglePlayerGame {
            game,
            on_send_view,
            on_send_error,
            player,
            agent,
            last_move: None,
            previous_states: vec![],
            request_ai_move,
            request_trainer_ranking,
            on_send_event,
            training_mode,
        };
        let against = format!("{:?}", agent);
        game.send_event(GameEvent::Start {
            training: game.training_mode,
            against,
        });
        game.agent_move();
        game.rank_moves();
        game.send_current_view();
        return game;
    }
}

impl SinglePlayerGame {
    fn agent_move(&mut self) {
        if self.game.get_turn() != Some(self.player.invert()) {
            log::info!("Not AI's turn");
            return;
        }
        let state = self.game.get_state();
        let msg = MoveRequest { state, agent: self.agent };
        let msg = JsValue::from_serde(&msg).unwrap();
        let this = JsValue::null();
        match self.request_ai_move.call1(&this, &msg) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to call request_ai_move: {:?}", err);
            }
        };
    }
    fn rank_moves(&mut self) {
        if !self.training_mode {
            return;
        }
        if self.game.get_turn() != Some(self.player) {
            log::info!("Not players turn (so not ranking moves)");
            return;
        }
        let state = self.game.get_state();
        let msg = JsValue::from_serde(&state).unwrap();
        let this = JsValue::null();
        match self.request_trainer_ranking.call1(&this, &msg) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to call request_trainer_ranking: {:?}", err);
            }
        };
    }
}

impl SinglePlayerGame {
    fn try_move(&mut self, game_move: Move) -> Result<(), String> {
        self.game.try_move(game_move)?;
        self.last_move = Some(game_move);
        self.send_current_view();
        match self.game.get_winner() {
            None => {}
            Some(winner) => {
                let winner = match winner == self.player {
                    true => "player".to_string(),
                    false => "ai".to_string(),
                };
                let against = format!("{:?}", self.agent);
                self.send_event(GameEvent::End {
                    training: self.training_mode,
                    against,
                    winner,
                })
            }
        };
        Ok(())
    }
    fn send_current_view(&self) {
        let view = GameView::from(&self.game.get_state());
        self.send_view(view);
    }
    fn send_view(&self, view: GameView) {
        let view = SinglePlayerView {
            player: self.player,
            game: view,
            last_move: self.last_move,
            can_undo: !self.previous_states.is_empty()
        };
        let view = JsValue::from_serde(&view).unwrap();
        let this = JsValue::null();
        match self.on_send_view.call1(&this, &view) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to call on_send_view: {:?}", err);
            }
        };
    }
    fn send_error(&self, error: String) {
        let error = JsValue::from(error);
        let this = JsValue::null();
        match self.on_send_error.call1(&this, &error) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to call on_send_error: {:?}", err);
            }
        };
    }
    fn send_event(&self, event: GameEvent) {
        let msg = ser::to_vec(&event).unwrap();
        let msg = serde_bytes::ByteBuf::from(msg);
        let msg = serde_wasm_bindgen::to_value(&msg).unwrap();
        let this = JsValue::null();
        match self.on_send_event.call1(&this, &msg) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

#[wasm_bindgen]
impl SinglePlayerGame {
    #[wasm_bindgen(js_name = move)]
    pub fn play_move(&mut self, game_move: &JsValue, is_player: bool) {
        let player_turn = self.game.get_turn() == Some(self.player);
        if player_turn != is_player {
            return self.send_error("Not your turn".to_string());
        }
        let current_state = PreviousState {
            game: self.game.clone(),
            last_move: self.last_move
        };
        let game_move: Move = match game_move.into_serde() {
            Ok(game_move) => game_move,
            Err(err) => {
                self.send_error(err.to_string());
                return;
            }
        };
        match self.try_move(game_move) {
            Ok(()) => {
                log::info!("Successfully played move");
            }
            Err(err) => {
                self.send_error(err);
                return;
            }
        };
        if is_player {
            self.previous_states.push(current_state);
        }
        self.agent_move();
        self.rank_moves();
    }

    #[wasm_bindgen(js_name = undo)]
    pub fn undo_move(&mut self) {
        let previous_state = match self.previous_states.pop() {
            Some(state) => state,
            None => {
                return;
            },
        };
        self.game = previous_state.game;
        self.last_move = previous_state.last_move;
        self.send_current_view();
        self.rank_moves();
    }

    pub fn reset(&mut self) {
        let against = format!("{:?}", self.agent);
        self.send_event(GameEvent::Start {
            training: self.training_mode,
            against,
        });
        self.game.reset();
        self.last_move = None;
        self.send_current_view();
        self.agent_move();
        self.rank_moves();
    }
}
