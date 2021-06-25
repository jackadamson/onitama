use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use crate::models::{GameState, Move, Player};
use crate::game::Game;
use web_sys::MessageEvent;
use serde_cbor::{ser,de};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Waiting,
    Running,
    Finished,
    RematchRequested,
    Closed,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameMessage {
    Joined,
    Initialize {
        state: GameState,
    },
    Move {
        game_move: Move,
    },
    RequestRematch,
}

#[wasm_bindgen]
pub struct MultiplayerGame {
    game: Game,
    on_send_msg: js_sys::Function,
    conn_state: ConnectionState,
    player: Player,
    is_host: bool,
}

#[wasm_bindgen]
impl MultiplayerGame {
    #[wasm_bindgen(constructor)]
    pub fn new(
        is_red: bool,
        is_host: bool,
        on_send_view: js_sys::Function,
        on_send_error: js_sys::Function,
        on_send_msg: js_sys::Function,
    ) -> MultiplayerGame {
        let game = Game::new(on_send_view, on_send_error);
        let player = match is_red {
            true => Player::Red,
            false => Player::Blue,
        };
        return MultiplayerGame {
            game,
            player,
            on_send_msg,
            is_host,
            conn_state: ConnectionState::Connecting,
        };
    }
}

#[wasm_bindgen]
impl MultiplayerGame {
    #[wasm_bindgen(js_name = move)]
    pub fn play_move(&mut self, game_move: &JsValue) {
        if !self.is_player_turn() {
            self.game.send_error("Not your turn".to_string());
            return;
        }
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
                let msg = GameMessage::Move { game_move };
                self.send_msg(msg);
            },
            Err(err) => {
                self.game.send_error(err);
            }
        };
    }
    pub fn reset(&mut self) {
        log::info!("Forgeiting");
        let game_move = Move::Forfeit;
        self.try_move(game_move).unwrap();
    }
    fn is_player_turn(&self) -> bool {
        match self.game.get_turn() {
            None => false,
            Some(turn) => turn == self.player,
        }
    }
}

impl MultiplayerGame {
    fn send_msg(&self, message: GameMessage) {
        log::info!("Sending msg");
        let msg = ser::to_vec(&message).unwrap();
        let msg = serde_bytes::ByteBuf::from(msg);
        let msg = serde_wasm_bindgen::to_value(&msg).unwrap();
        log::info!("Serialized: {:?}", &msg);
        let this = JsValue::null();
        log::info!("Message sent");
        match self.on_send_msg.call1(&this, &msg) {
            Ok(_) => {},
            Err(err) => {
                log::error!("Failed to call on_send_msg: {:?}", err);
            },
        };
    }
}

#[wasm_bindgen]
impl MultiplayerGame {
    #[wasm_bindgen(js_name = connected)]
    pub fn connected(&mut self) {
        if self.conn_state != ConnectionState::Connecting {
            log::error!("Connected called while already connected");
            return;
        }
        self.conn_state = ConnectionState::Waiting;
        if !self.is_host {
            let msg = GameMessage::Joined;
            self.send_msg(msg);
        }
    }
    #[wasm_bindgen(js_name = handleMsg)]
    pub fn handle_message(&mut self, msg: MessageEvent) {
        if let Ok(msg) = msg.data().dyn_into::<js_sys::ArrayBuffer>() {
            log::info!("Received an array buffer");
            let msg = js_sys::Uint8Array::new(&msg).to_vec();
            let msg: GameMessage = match de::from_slice(&msg) {
                Ok(message) => message,
                Err(err) => {
                    log::error!("Failed to decode message: {:?}", err);
                    return;
                },
            };
            self.handle_game_message(msg);
        } else {
            log::info!("Received unexpected msg type: {:?}", &msg);
        }
    }
    fn try_move(&mut self, game_move: Move) -> Result<(), String> {
        self.game.try_move(game_move)?;
        if self.game.is_finished() {
            self.conn_state = ConnectionState::Finished;
        }
        Ok(())
    }
    fn handle_game_message(&mut self, msg: GameMessage) {
        // TODO: Harden against malicious stuff
        match (self.conn_state, msg) {
            (ConnectionState::Waiting, GameMessage::Initialize { state }) => {
                log::info!("Initializing");
                self.conn_state = ConnectionState::Running;
                self.game.set_state(state);
                self.game.send_current_view();
            },
            (ConnectionState::Waiting, GameMessage::Joined) => {
                log::info!("Player joined");
                self.conn_state = ConnectionState::Running;
                self.game.send_current_view();
                let state = self.game.get_state();
                let msg = GameMessage::Initialize { state };
                self.send_msg(msg);
            },
            (ConnectionState::Running, GameMessage::Move { game_move }) => {
                log::info!("Received move");
                if self.is_player_turn() {
                    log::error!("Opponent attempted to play during our turn");
                    self.game.send_error("Opponent played out of turn".to_string());
                    return;
                }
                match self.game.try_move(game_move) {
                    Ok(()) => {}
                    Err(err) => {
                        log::error!("Opponent played illegal move: {}", err);
                        self.game.send_error("Opponent Played Illegal Move".to_string());
                    },
                }
            }
            (state, msg) => {
                log::error!("Illegal state transition state = {:?}, message = {:?}", state, msg)
            },
        }
    }
}
