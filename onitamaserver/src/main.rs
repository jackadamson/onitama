// use actix::{Actor, Addr, Context, StreamHandler};
use actix::prelude::*;
use actix_web::{error, App, HttpServer, web, HttpRequest, HttpResponse, Error, ResponseError};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::actors::{OnitamaServer,OnitamaWs};
use std::str::FromStr;

mod queues;
mod actors;
mod messages;

// async fn join_room() -> impl Responder {
//     "Hello world!"
// }
async fn join_room(
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

async fn create_room(
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

struct ServerData {
    pub server_addr: Addr<OnitamaServer>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_addr = OnitamaServer::new().start();
    let data = ServerData { server_addr };
    let data = web::Data::new(data);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/ws/{key}", web::get().to(join_room))
            .route("/ws/", web::get().to(create_room))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
