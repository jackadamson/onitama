use actix::prelude::*;
use actix_web::web::Bytes;
use uuid::Uuid;

use crate::actors::{OnitamaRoom, OnitamaWs};

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub addr: Addr<OnitamaWs>,
    pub room_key: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateRoom(pub Addr<OnitamaWs>);

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
    pub sender: Addr<OnitamaWs>,
    pub data: GameData,
}
