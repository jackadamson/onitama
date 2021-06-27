use actix::prelude::*;
use actix_web::web::Bytes;
use uuid::Uuid;

use crate::rooms::{OnitamaRoom, RoomWs};
use crate::agents::{AgentException, AgentWs};
use onitamalib::{GameMessage, GameState};
use onitamalib::models::Player;

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
    pub player: Player,
    pub initial_state: GameState,
}

#[derive(Debug)]
pub enum GameData {
    Binary(Bytes),
    Text(String),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddressedGameMessage {
    pub sender: Addr<RoomWs>,
    pub msg: GameMessage,
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
