use actix::{Addr, Message};
use uuid::Uuid;

use onitamalib::{GameMessage, GameState, Player};

use crate::agents::{AgentException, AgentWs};
use crate::rooms::{OnitamaRoom, RoomWs};

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
pub struct LeftRoom(pub Addr<RoomWs>);

#[derive(Message)]
#[rtype(result = "()")]
pub enum JoinedRoom {
    Success {
        addr: Addr<OnitamaRoom>,
        room_key: Uuid,
        player: Player,
        state: GameState,
        waiting: bool,
    },
    Error {
        message: String,
    },
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddressedGameMessage {
    pub sender: Addr<RoomWs>,
    pub msg: GameMessage,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketGameMessage(pub GameMessage);

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseRoom;
