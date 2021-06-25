use actix::prelude::*;
use actix_web::{App, HttpServer, web};

use crate::rooms::OnitamaServer;
use crate::routes::{ServerData,join_room,create_room,ai_room};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod rooms;
mod messages;
mod routes;
mod agents;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let server_addr = OnitamaServer::new().start();
    let data = ServerData { server_addr };
    let data = web::Data::new(data);
    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/ws/ai", web::get().to(ai_room))
            .route("/ws/{key}", web::get().to(join_room))
            .route("/ws/", web::get().to(create_room))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
