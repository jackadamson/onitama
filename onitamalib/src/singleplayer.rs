use wasm_bindgen::prelude::*;
use crate::models::{GameState, GameView, Move};
use crate::game::Game;

#[wasm_bindgen]
pub struct SingleplayerGame {
    game: Game,
}

#[wasm_bindgen]
impl SingleplayerGame {
    #[wasm_bindgen(constructor)]
    pub fn new(on_send_view: js_sys::Function, on_send_error: js_sys::Function) -> SingleplayerGame {
        let game = Game::new(on_send_view, on_send_error);
        return SingleplayerGame { game };
    }
}

#[wasm_bindgen]
impl SingleplayerGame {
    #[wasm_bindgen(js_name = move)]
    pub fn play_move(&mut self, game_move: &JsValue) {
        match self.game.try_move(game_move) {
            Ok(state) => {
                self.game.set_state(state);
                self.game.send_current_view();
                log::info!("Successfully played move");
            },
            Err(err) => {
                self.game.send_error(err);
            }
        };
    }
    pub fn reset(&mut self) {
        self.game.reset();
    }
}
