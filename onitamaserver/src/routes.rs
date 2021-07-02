use std::str::FromStr;

use actix::prelude::*;
use actix_web::{error, Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::agents::AgentWs;
use crate::rooms::{OnitamaServer, RoomWs};
use crate::utils::get_identifier;

pub async fn join_room(
    req: HttpRequest,
    web::Path(key): web::Path<String>,
    stream: web::Payload,
    data: web::Data<ServerData>,
) -> Result<HttpResponse, Error> {
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
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let id = get_identifier(&req);
    info!("AI Game Start: {}", &id);
    let actor = AgentWs::new(id);
    let resp = ws::start(actor, &req, stream);
    resp
}

pub struct ServerData {
    pub server_addr: Addr<OnitamaServer>,
}
