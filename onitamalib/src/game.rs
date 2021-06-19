use wasm_bindgen::prelude::*;
use crate::models::{GameState, GameView, Move};

pub struct Game {
    state: GameState,
    on_send_view: js_sys::Function,
    on_send_error: js_sys::Function,
}

impl Game {
    pub fn new(on_send_view: js_sys::Function, on_send_error: js_sys::Function) -> Game {
        let game = Game {
            state: GameState::new(),
            on_send_view,
            on_send_error,
        };
        let view = GameView::from(&game.state);
        game.send_view(view);
        return game;
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.state = GameState::new();
        self.send_current_view();
    }
}

impl Game {
    pub fn send_current_view(&self) {
        let view = GameView::from(&self.state);
        self.send_view(view);
    }
    pub fn send_view(&self, view: GameView) {
        let view = JsValue::from_serde(&view).unwrap();
        let this = JsValue::null();
        match self.on_send_view.call1(&this, &view) {
            Ok(_) => {},
            Err(err) => {
                log::error!("Failed to call on_send_view: {:?}", err);
            },
        };
    }
    pub fn send_error(&self, error: String) {
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

impl Game {
    pub fn try_move(&self, game_move: &JsValue) -> Result<GameState, String> {
        let game_move: Move = match game_move.into_serde() {
            Ok(game_move) => game_move,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let board = match &self.state {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return Err("Game Already Finished".to_string());
            },
        };
        board.make_move(game_move)
    }
    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }
}
