use std::str::FromStr;

use actix::prelude::*;
use actix_web::{error, Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use uuid::Uuid;

use onitamalib::AiAgent;

use crate::agents::AgentWs;
use crate::rooms::{OnitamaServer, RoomWs};
use crate::utils::get_identifier;

pub async fn join_room(
    req: HttpRequest,
    path: web::Path<String>,
    stream: web::Payload,
    data: web::Data<ServerData>,
) -> Result<HttpResponse, Error> {
    let key = path.to_string();
    let server: Addr<OnitamaServer> = data.server_addr.clone();
    let id = get_identifier(&req);
    let key = match Uuid::from_str(&key) {
        Ok(key) => key,
        Err(_) => {
            return Err(error::ErrorBadRequest("Invalid UUID"));
        }
    };
    let actor = RoomWs::new(server, Some(key), id);
    let resp = ws::start(actor, &req, stream);
    resp
}

pub async fn create_room(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<ServerData>,
) -> Result<HttpResponse, Error> {
    let id = get_identifier(&req);
    let server: Addr<OnitamaServer> = data.server_addr.clone();
    let actor = RoomWs::new(server, None, id);
    let resp = ws::start(actor, &req, stream);
    resp
}

pub async fn ai_room(
    req: HttpRequest,
    difficulty: web::Path<String>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let id = get_identifier(&req);
    let difficulty = difficulty.as_str();
    let ai = match difficulty {
        "easy" => AiAgent::Greedy,
        "medium" => AiAgent::PureMonteCarlo,
        "hard" => AiAgent::HybridMonteCarlo,
        _ => AiAgent::PureMonteCarlo,
    };
    info!("AI Game Start: {}, ({:?})", &id, ai);
    let actor = AgentWs::new(id, ai);
    let resp = ws::start(actor, &req, stream);
    resp
}

pub struct ServerData {
    pub server_addr: Addr<OnitamaServer>,
}
