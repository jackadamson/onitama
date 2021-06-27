use std::collections::HashMap;

use actix::prelude::*;
use actix_web::web::Bytes;
use actix_web_actors::ws;
use serde_cbor::ser;
use uuid::Uuid;

use onitamalib::{GameMessage, GameState};

use crate::messages::{AddressedGameMessage, CreateRoom, GameData, JoinedRoom, JoinRoom};
use onitamalib::models::Player;

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
            },
            Ok(ws::Message::Binary(data)) => data,
            _ => {
                warn!("Received unexpected data-type");
                return;
            },
        };
        let msg: GameMessage = match serde_cbor::from_slice(data.as_ref()) {
            Ok(msg) => msg,
            Err(err) => {
                warn!("Error deserializing player message: {:?}", err);
                return;
            },
        };
        let room = match &self.room {
            Some(room) => room,
            None => {
                warn!("Message sent too early");
                ctx.text("Error: Message too early");
                return;
            },
        };
        let msg = AddressedGameMessage { sender: ctx.address(), msg };
        room.do_send(msg);
    }
}

impl Handler<JoinedRoom> for RoomWs {
    type Result = ();
    fn handle(&mut self, msg: JoinedRoom, ctx: &mut Self::Context) {
        let JoinedRoom{ addr, room_key, player, initial_state } = msg;
        self.room = Some(addr);
        self.room_key = Some(room_key);
        let msg = GameMessage::Initialize {
            state: initial_state,
            room_id: room_key.to_string(),
            player,
            waiting: player == Player::Red,
        };
        let msg = ser::to_vec(&msg).expect("failed to serialize initialize message");
        ctx.binary(msg);
    }
}

impl Handler<AddressedGameMessage> for RoomWs {
    type Result = ();
    fn handle(&mut self, msg: AddressedGameMessage, ctx: &mut Self::Context) {
        let AddressedGameMessage { msg, .. } = msg;
        let data = ser::to_vec(&msg).expect("Failed to serialize message");
        ctx.binary(data);
    }
}

/// Room
///
pub struct OnitamaRoom {
    initial_state: GameState,
    sockets: Vec<Addr<RoomWs>>,
    key: Uuid,
}

impl OnitamaRoom {
    pub fn new() -> OnitamaRoom {
        OnitamaRoom {
            initial_state: GameState::new(),
            sockets: vec![],
            key: Uuid::new_v4(),
        }
    }
}

impl Actor for OnitamaRoom {
    type Context = Context<Self>;
}

impl Handler<JoinRoom> for OnitamaRoom {
    type Result = ();
    fn handle(&mut self, msg: JoinRoom, ctx: &mut Self::Context) {
        let socket = msg.addr;
        let player = match self.sockets.len() == 0 {
            true => Player::Red,
            false => Player::Blue,
        };
        self.sockets.push(socket.clone());
        let addr = ctx.address();
        let room_key = self.key;
        let msg = JoinedRoom {
            addr,
            room_key,
            player,
            initial_state: self.initial_state
        };
        socket.do_send(msg);
        if player == Player::Blue {
            // Send join message
            let socket = self.sockets.get(0).expect("No sockets");
            let msg = AddressedGameMessage { sender: socket.clone(), msg: GameMessage::Joined };
            socket.do_send(msg);
        }
    }
}

impl Handler<AddressedGameMessage> for OnitamaRoom {
    type Result = ();
    fn handle(&mut self, msg: AddressedGameMessage, _ctx: &mut Self::Context) {
        for addr in self.sockets.iter() {
            if *addr != msg.sender {
                addr.do_send(msg);
                return;
            }
        }
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
        let room = match self.rooms.get(room_key) {
            None => {
                return;
            }
            Some(room) => room,
        };
        room.do_send(msg);
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
