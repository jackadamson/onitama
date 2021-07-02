use std::collections::HashMap;

use actix::prelude::*;
use actix_web_actors::ws;
use rand::prelude::*;
use serde_cbor::ser;
use uuid::Uuid;

use onitamalib::{GameMessage, GameState, Move};
use onitamalib::models::Player;

use crate::messages::{
    AddressedGameMessage, CreateRoom, JoinedRoom, JoinRoom, LeftRoom, SocketGameMessage
};

/// Socket
///
pub struct RoomWs {
    room: Option<Addr<OnitamaRoom>>,
    server: Addr<OnitamaServer>,
    room_key: Option<Uuid>,
}

impl RoomWs {
    pub fn new(server: Addr<OnitamaServer>, room_key: Option<Uuid>) -> RoomWs {
        RoomWs {
            room: None,
            server,
            room_key,
        }
    }
}

impl Actor for RoomWs {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is alive");
        let addr = ctx.address();
        match self.room_key {
            None => {
                println!("Creating a room");
                let msg = CreateRoom(addr);
                self.server.do_send(msg);
            }
            Some(room_key) => {
                println!("Joining a room");
                let msg = JoinRoom { addr, room_key };
                self.server.do_send(msg);
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RoomWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let data = match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
                return;
            }
            Ok(ws::Message::Binary(data)) => data,
            Ok(ws::Message::Close(reason)) => {
                info!("Connection closed, reason: {:?}", reason);
                if let Some(room) = &self.room {
                    room.do_send(LeftRoom(ctx.address()));
                }
                return;
            }
            msg => {
                warn!("Received unexpected msg: {:?}", msg);
                return;
            }
        };
        let msg: GameMessage = match serde_cbor::from_slice(data.as_ref()) {
            Ok(msg) => msg,
            Err(err) => {
                warn!("Error deserializing player message: {:?}", err);
                return;
            }
        };
        let room = match &self.room {
            Some(room) => room,
            None => {
                warn!("Message sent too early");
                ctx.text("Error: Message too early");
                return;
            }
        };
        let msg = AddressedGameMessage { sender: ctx.address(), msg };
        room.do_send(msg);
    }
}

impl Handler<JoinedRoom> for RoomWs {
    type Result = ();
    fn handle(&mut self, msg: JoinedRoom, ctx: &mut Self::Context) {
        let msg = match msg {
            JoinedRoom::Error { message } => {
                GameMessage::Error { message }
            }
            JoinedRoom::Success { addr, room_key, player, state, waiting } => {
                self.room = Some(addr);
                self.room_key = Some(room_key);
                GameMessage::Initialize {
                    state,
                    room_id: room_key.to_string(),
                    player,
                    waiting,
                }
            }
        };
        let msg = ser::to_vec(&msg).expect("failed to serialize initialize message");
        ctx.binary(msg);
    }
}

impl Handler<SocketGameMessage> for RoomWs {
    type Result = ();
    fn handle(&mut self, msg: SocketGameMessage, ctx: &mut Self::Context) {
        let SocketGameMessage(msg) = msg;
        let data = ser::to_vec(&msg).expect("Failed to serialize message");
        ctx.binary(data);
    }
}

/// Room
///
pub struct OnitamaRoom {
    game_state: GameState,
    red: Option<Addr<RoomWs>>,
    blue: Option<Addr<RoomWs>>,
    key: Uuid,
    requested_rematch: Option<Player>,
}

impl OnitamaRoom {
    pub fn new() -> OnitamaRoom {
        OnitamaRoom {
            game_state: GameState::new(),
            red: None,
            blue: None,
            key: Uuid::new_v4(),
            requested_rematch: None,
        }
    }
}

impl Actor for OnitamaRoom {
    type Context = Context<Self>;
}

impl OnitamaRoom {
    fn send_to_player(&self, player: Player, msg: GameMessage) {
        let msg = SocketGameMessage(msg);
        let sock = match player {
            Player::Red => &self.red,
            Player::Blue => &self.blue,
        };
        match sock {
            None => {}
            Some(sock) => {
                sock.do_send(msg);
            }
        };
    }
    fn player_from_addr(&self, addr: &Addr<RoomWs>) -> Option<Player> {
        if Some(addr) == self.red.as_ref() {
            return Some(Player::Red);
        }
        if Some(addr) == self.blue.as_ref() {
            return Some(Player::Blue);
        }
        return None;
    }
    fn broadcast(&self, msg: GameMessage) {
        if let Some(sock) = &self.blue {
            let msg = SocketGameMessage(msg.clone());
            sock.do_send(msg);
        }
        if let Some(sock) = &self.red {
            let msg = SocketGameMessage(msg.clone());
            sock.do_send(msg);
        }
    }
}

impl Handler<JoinRoom> for OnitamaRoom {
    type Result = ();
    fn handle(&mut self, msg: JoinRoom, ctx: &mut Self::Context) {
        let socket = msg.addr;
        let mut waiting = false;
        let player: Player = match (&self.red, &self.blue) {
            (None, None) => {
                waiting = true;
                let mut rng = thread_rng();
                match rng.gen() {
                    true => Player::Red,
                    false => Player::Blue,
                }
            }
            (Some(_), None) => Player::Blue,
            (None, Some(_)) => Player::Red,
            (Some(_), Some(_)) => {
                let message = "Room is full".to_string();
                let msg = JoinedRoom::Error { message };
                socket.do_send(msg);
                return;
            }
        };
        info!("Player joined: {:?}", player);
        match player {
            Player::Red => { self.red = Some(socket.clone()); }
            Player::Blue => { self.blue = Some(socket.clone()); }
        };
        let addr = ctx.address();
        let room_key = self.key;
        let msg = JoinedRoom::Success {
            addr,
            room_key,
            player,
            waiting,
            state: self.game_state,
        };
        socket.do_send(msg);
        // Send join message
        self.send_to_player(player.invert(), GameMessage::Joined);
    }
}

impl Handler<LeftRoom> for OnitamaRoom {
    type Result = ();
    fn handle(&mut self, msg: LeftRoom, ctx: &mut Self::Context) {
        info!("handling left room");
        let LeftRoom(addr) = msg;
        if self.blue.as_ref() == Some(&addr) {
            self.blue = None;
        }
        if self.red.as_ref() == Some(&addr) {
            self.red = None;
        }
        match (&self.blue, &self.red) {
            (None, None) => {
                info!("Room Closing: {}", self.key.clone());
                ctx.stop();
            },
            _ => {
                self.broadcast(GameMessage::Disconnected);
            }
        };
    }
}

impl OnitamaRoom {
    fn handle_move(&mut self, game_move: Move, player: Player) {
        let board = match self.game_state {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                info!("Attempted move on finished game");
                return;
            }
        };
        if board.turn != player {
            error!("Not player's turn");
            return;
        }
        let new_state = match board.try_move(game_move) {
            Ok(new_state) => new_state,
            Err(err) => {
                error!("Player played illegal move: {:?}", err);
                return;
            }
        };
        self.game_state = new_state;
        let next_player = player.invert();
        let msg = GameMessage::Move { game_move };
        self.send_to_player(next_player, msg);
    }
    fn handle_rematch_request(&mut self, player: Player) {
        let requested_player = match self.requested_rematch {
            None => {
                self.requested_rematch = Some(player);
                let msg = GameMessage::RequestRematch;
                let other_player = player.invert();
                self.send_to_player(other_player, msg);
                return;
            }
            Some(requested_player) => requested_player,
        };
        if requested_player == player {
            info!("Player requsted rematch multiple times");
        } else {
            self.requested_rematch = None;
            let state = GameState::new();
            self.game_state = state;
            self.send_to_player(Player::Red, GameMessage::Initialize {
                state,
                room_id: self.key.to_string(),
                player: Player::Red,
                waiting: false,
            });
            self.send_to_player(Player::Blue, GameMessage::Initialize {
                state,
                room_id: self.key.to_string(),
                player: Player::Blue,
                waiting: false,
            });
        }
    }
}

impl Handler<AddressedGameMessage> for OnitamaRoom {
    type Result = ();
    fn handle(&mut self, msg: AddressedGameMessage, _ctx: &mut Self::Context) {
        let AddressedGameMessage { sender, msg } = msg;
        let player = match self.player_from_addr(&sender) {
            Some(player) => player,
            None => {
                error!("Received game message from socket that's not in game");
                let msg = GameMessage::Error { message: "Player not in game".to_string() };
                sender.do_send(SocketGameMessage(msg));
                return;
            }
        };
        match msg {
            GameMessage::Move { game_move } => {
                self.handle_move(game_move, player);
            }
            GameMessage::RequestRematch => {
                self.handle_rematch_request(player);
            }
            GameMessage::Error { message } => {
                error!("Received error from client: {}", message);
                return;
            }
            msg => {
                info!("Unexpected msg: {:?}", msg);
                return;
            }
        };
    }
}

/// Server
///
pub struct OnitamaServer {
    rooms: HashMap<Uuid, Addr<OnitamaRoom>>,
}

impl OnitamaServer {
    pub fn new() -> OnitamaServer {
        OnitamaServer {
            rooms: HashMap::new(),
        }
    }
}

impl Actor for OnitamaServer {
    type Context = Context<Self>;
}

impl Handler<JoinRoom> for OnitamaServer {
    type Result = ();
    fn handle(&mut self, msg: JoinRoom, _: &mut Self::Context) {
        println!("Server received join room request");
        let room_key = &msg.room_key;
        let addr = msg.addr.clone();
        let err_resp = GameMessage::Error { message: "Requested room doesn't exist".to_string() };
        let room = match self.rooms.get(room_key) {
            None => {
                addr.do_send(SocketGameMessage(err_resp));
                return;
            }
            Some(room) => room,
        };
        match room.try_send(msg) {
            Ok(_) => {}
            Err(_) => {
                addr.do_send(SocketGameMessage(err_resp));
            }
        };
    }
}

impl Handler<CreateRoom> for OnitamaServer {
    type Result = ();
    fn handle(&mut self, msg: CreateRoom, _: &mut Self::Context) {
        println!("Server received create room request");
        let room = OnitamaRoom::new();
        let room_key = room.key;
        let room = room.start();
        self.rooms.insert(room_key, room.clone());
        let msg = JoinRoom {
            addr: msg.0,
            room_key,
        };
        room.do_send(msg);
    }
}
