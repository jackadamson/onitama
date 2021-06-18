use std::str::FromStr;

use actix::prelude::*;
use actix_web::{error, Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::actors::{OnitamaServer, OnitamaWs};

pub async fn join_room(
    req: HttpRequest,
    web::Path(key): web::Path<String>,
    stream: web::Payload,
    data: web::Data<ServerData>,
) -> Result<HttpResponse, Error> {
    let server: Addr<OnitamaServer> = data.server_addr.clone();
    let key = match Uuid::from_str(&key) {
        Ok(key) => key,
        Err(_) => {
            return Err(error::ErrorBadRequest("Invalid UUID"));
        }
    };
    let actor = OnitamaWs::new(server, Some(key));
    let resp = ws::start(actor, &req, stream);
    println!("{:?}", resp);
    resp
}

pub async fn create_room(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<ServerData>,
) -> Result<HttpResponse, Error> {
    let server: Addr<OnitamaServer> = data.server_addr.clone();
    let actor = OnitamaWs::new(server, None);
    let resp = ws::start(actor, &req, stream);
    println!("{:?}", resp);
    resp
}

pub struct ServerData {
    pub server_addr: Addr<OnitamaServer>,
}
