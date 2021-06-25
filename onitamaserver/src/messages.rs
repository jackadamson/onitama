use actix::prelude::*;
use actix_web::web::Bytes;
use uuid::Uuid;

use crate::rooms::{OnitamaRoom, RoomWs};
use crate::agents::{AgentException, AgentWs};
use onitamalib::GameMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub addr: Addr<RoomWs>,
    pub room_key: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateRoom(pub Addr<RoomWs>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinedRoom {
    pub addr: Addr<OnitamaRoom>,
    pub room_key: Uuid,
}

#[derive(Debug)]
pub enum GameData {
    Binary(Bytes),
    Text(String),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddressedGameData {
    pub sender: Addr<RoomWs>,
    pub data: GameData,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AgentRequest {
    pub msg: GameMessage,
    pub addr: Addr<AgentWs>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AgentResponse {
    pub resp: Result<GameMessage,AgentException>,
}
