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
    on_send_view: js_sys::Function,
    on_send_error: js_sys::Function,
    on_send_event: js_sys::Function,
    request_ai_move: js_sys::Function,
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
        difficulty: &str, on_send_view: js_sys::Function, on_send_error: js_sys::Function, request_ai_move: js_sys::Function, on_send_event: js_sys::Function,
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
            on_send_event,
        };
        let against = format!("{:?}", agent);
        game.send_event(GameEvent::Start { against });
        game.agent_move();
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
    }

    pub fn reset(&mut self) {
        let against = format!("{:?}", self.agent);
        self.send_event(GameEvent::Start { against });
        self.game.reset();
        self.last_move = None;
        self.send_current_view();
        self.agent_move();
    }
}
