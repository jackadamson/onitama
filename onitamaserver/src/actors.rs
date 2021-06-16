use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;
use std::collections::HashMap;
use crate::messages::{CreateRoom, JoinRoom, JoinedRoom};

/// Socket
/// 
pub struct OnitamaWs {
    room: Option<Addr<OnitamaRoom>>,
    server: Addr<OnitamaServer>,
    room_key: Option<Uuid>,
}
impl OnitamaWs {
    pub fn new(server: Addr<OnitamaServer>, room_key: Option<Uuid>) -> OnitamaWs {
        OnitamaWs {
            room: None,
            server,
            room_key,
        }
    }
}
impl Actor for OnitamaWs {
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for OnitamaWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<JoinedRoom> for OnitamaWs {
    type Result = ();
    fn handle(&mut self, msg: JoinedRoom, ctx: &mut Self::Context) {
        let addr = msg.addr;
        self.room = Some(addr);
        self.room_key = Some(msg.room_key);
        ctx.text(msg.room_key.to_string());
    }
}

/// Room
///
pub struct OnitamaRoom {
    sockets: Vec<Addr<OnitamaWs>>,
    key: Uuid,
}

impl OnitamaRoom {
    pub fn new() -> OnitamaRoom {
        OnitamaRoom {
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
        self.sockets.push(socket.clone());
        let addr = ctx.address();
        let room_key = self.key;
        let msg = JoinedRoom {
            addr,
            room_key,
        };
        socket.do_send(msg);
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

// pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let resp = ws::start(OnitamaWs {}, &req, stream);
//     println!("{:?}", resp);
//     resp
// }
