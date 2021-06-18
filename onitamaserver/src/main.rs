use actix::prelude::*;
use actix_web::{App, HttpServer, web};

use crate::actors::OnitamaServer;
use crate::routes::{ServerData,join_room,create_room};

mod actors;
mod messages;
mod routes;


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
