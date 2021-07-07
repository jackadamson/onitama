use wasm_bindgen::prelude::*;
use rand::prelude::*;
use serde::Serialize;

use crate::{AiAgent, GameView, Player};
use crate::gamemodes::base::Game;
use crate::models::Move;
use instant::Duration;

#[wasm_bindgen]
pub struct SinglePlayerGame {
    game: Game,
    player: Player,
    agent: AiAgent,
    last_move: Option<Move>,
    on_send_view: js_sys::Function,
    on_send_error: js_sys::Function,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SinglePlayerView {
    player: Player,
    #[serde(flatten)]
    game: GameView,
    last_move: Option<Move>,
}

#[wasm_bindgen]
impl SinglePlayerGame {
    #[wasm_bindgen(constructor)]
    pub fn new(difficulty: &str, on_send_view: js_sys::Function, on_send_error: js_sys::Function) -> SinglePlayerGame {
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
            game, on_send_view, on_send_error, player, agent, last_move: None
        };
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
        let duration = Duration::from_millis(50);
        let (game_move, expected) = match self.agent.play_move(&state, duration) {
            None => {
                self.send_error("AI failed to play".to_string());
                return;
            }
            Some(game_move) => game_move,
        };
        log::info!("Expected score {} with move {:?}", expected, game_move);
        self.try_move(game_move).unwrap();
    }
}
impl SinglePlayerGame {
    fn try_move(&mut self, game_move: Move) -> Result<(), String> {
        self.game.try_move(game_move)?;
        self.last_move = Some(game_move);
        self.send_current_view();
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
        };
        let view = JsValue::from_serde(&view).unwrap();
        let this = JsValue::null();
        match self.on_send_view.call1(&this, &view) {
            Ok(_) => {},
            Err(err) => {
                log::error!("Failed to call on_send_view: {:?}", err);
            },
        };
    }
    fn send_error(&self, error: String) {
        let error = JsValue::from(error);
        let this = JsValue::null();
        match self.on_send_error.call1(&this, &error) {
            Ok(_) => {},
            Err(err) => {
                log::error!("Failed to call on_send_error: {:?}", err);
            },
        };
    }
}

#[wasm_bindgen]
impl SinglePlayerGame {
    #[wasm_bindgen(js_name = move)]
    pub fn play_move(&mut self, game_move: &JsValue) {
        if self.game.get_turn() != Some(self.player) {
            return self.send_error("Not your turn".to_string());
        }
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
            },
            Err(err) => {
                self.send_error(err);
                return;
            }
        };
        self.agent_move();
    }
    pub fn reset(&mut self) {
        self.game.reset();
        self.send_current_view();
    }
}
