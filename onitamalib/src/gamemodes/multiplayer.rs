use serde::Serialize;
use serde_cbor::{de, ser};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

use crate::gamemodes::base::Game;
use crate::GameView;
use crate::messages::GameMessage;
use crate::models::{Move, Player};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ConnectionState {
    Connecting,
    Waiting,
    Running,
    Finished,
    RematchRequested,
    OpponentRematchRequested,
    OpponentDisconnected,
    Disconnected,
    Errored,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiplayerView {
    connection: ConnectionState,
    player: Player,
    error: Option<String>,
    room_id: Option<String>,
    #[serde(flatten)]
    game: GameView,
    last_move: Option<Move>,
}

#[wasm_bindgen]
pub struct MultiplayerGame {
    game: Game,
    on_send_msg: js_sys::Function,
    on_send_view: js_sys::Function,
    on_send_error: js_sys::Function,
    conn_state: ConnectionState,
    player: Player,
    room_id: Option<String>,
    error: Option<String>,
}

impl MultiplayerGame {
    fn send_current_view(&self) {
        let game = GameView::from(&self.game.get_state());
        let view = MultiplayerView {
            game,
            connection:
            self.conn_state,
            room_id: self.room_id.clone(),
            player: self.player,
            error: self.error.clone(),
            last_move: self.game.get_last_move(),
        };
        self.send_view(view);
    }
    fn send_view(&self, view: MultiplayerView) {
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
impl MultiplayerGame {
    #[wasm_bindgen(constructor)]
    pub fn new(
        on_send_view: js_sys::Function,
        on_send_error: js_sys::Function,
        on_send_msg: js_sys::Function,
    ) -> MultiplayerGame {
        let game = Game::new();
        let game = MultiplayerGame {
            room_id: None,
            game,
            player: Player::Red, // Start Red, changes once playing
            on_send_msg,
            on_send_view,
            on_send_error,
            conn_state: ConnectionState::Connecting,
            error: None,
        };
        game.send_current_view();
        return game;
    }
}

#[wasm_bindgen]
impl MultiplayerGame {
    #[wasm_bindgen(js_name = move)]
    pub fn play_move(&mut self, game_move: &JsValue) {
        if !self.is_player_turn() {
            self.send_error("Not your turn".to_string());
            return;
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
                let msg = GameMessage::Move { game_move };
                self.send_msg(msg);
            },
            Err(err) => {
                self.send_error(err);
            }
        };
    }
    pub fn reset(&mut self) {
        log::info!("Forgeiting");
        self.conn_state = ConnectionState::RematchRequested;
        let msg = GameMessage::RequestRematch;
        self.send_msg(msg);
        self.send_current_view();
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
        let msg = ser::to_vec(&message).unwrap();
        let msg = serde_bytes::ByteBuf::from(msg);
        let msg = serde_wasm_bindgen::to_value(&msg).unwrap();
        let this = JsValue::null();
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
    #[wasm_bindgen(js_name = handleMsg)]
    pub fn handle_message(&mut self, msg: MessageEvent) {
        if let Ok(msg) = msg.data().dyn_into::<js_sys::ArrayBuffer>() {
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
        self.send_current_view();
        Ok(())
    }
    fn handle_game_message(&mut self, msg: GameMessage) {
        log::info!("Message: {:?}", &msg);
        match (self.conn_state, msg) {
            (ConnectionState::Connecting | ConnectionState::RematchRequested,
                GameMessage::Initialize { state, room_id, player, waiting }) => {
                log::info!("Initializing");
                self.room_id = Some(room_id);
                self.player = player;
                self.conn_state = match waiting {
                    true => ConnectionState::Waiting,
                    false => ConnectionState::Running,
                };
                self.game.set_state(state);
            },
            (ConnectionState::Waiting | ConnectionState::OpponentDisconnected, GameMessage::Joined) => {
                log::info!("Player joined");
                self.conn_state = ConnectionState::Running;
            },
            (ConnectionState::Running, GameMessage::Move { game_move }) => {
                log::info!("Received move");
                if self.is_player_turn() {
                    log::error!("Opponent attempted to play during our turn");
                    self.send_error("Opponent played out of turn".to_string());
                    return;
                }
                match self.try_move(game_move) {
                    Ok(()) => {}
                    Err(err) => {
                        log::error!("Opponent played illegal move: {}", err);
                        self.send_error("Opponent played illegal move".to_string());
                    },
                }
            }
            (ConnectionState::Finished, GameMessage::RequestRematch) => {
                self.conn_state = ConnectionState::OpponentRematchRequested;
            },
            (_, GameMessage::Disconnected) => {
                self.conn_state = ConnectionState::OpponentDisconnected;
            },
            (_, GameMessage::Error { message }) => {
                self.conn_state = ConnectionState::Errored;
                self.error = Some(message);
            },
            (state, msg) => {
                log::error!("Illegal state transition state = {:?}, message = {:?}", state, msg)
            },
        }
        self.send_current_view();
    }
}
