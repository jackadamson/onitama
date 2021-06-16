use actix::prelude::*;
use crate::actors::{OnitamaServer,OnitamaWs,OnitamaRoom};
use uuid::Uuid;

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
