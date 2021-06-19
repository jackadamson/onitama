use wasm_bindgen::prelude::*;
use crate::models::Move;
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
        let game_move: Move = match game_move.into_serde() {
            Ok(game_move) => game_move,
            Err(err) => {
                self.game.send_error(err.to_string());
                return;
            }
        };
        match self.game.try_move(game_move) {
            Ok(()) => {
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
