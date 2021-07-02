#[macro_use] extern crate log;
extern crate pretty_env_logger;

use std::path;

use actix::prelude::*;
use actix_files::Files;
use actix_web::{App, HttpServer, web};

use crate::rooms::OnitamaServer;
use crate::routes::{ai_room, create_room, join_room, ServerData};


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
    let mut built_path = path::Path::new("./build");
    if !built_path.exists() {
        built_path = path::Path::new("../build");
    }
    info!("Does build path exist ({}): {}", built_path.as_os_str().to_string_lossy(),  built_path.exists());
    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/ws")
                    .route("/ai", web::get().to(ai_room))
                    .route("/{key}", web::get().to(join_room))
                    .route("/", web::get().to(create_room))
            )
            .service(
                Files::new(
                    "/",
                    built_path,
                )
                    .index_file("index.html")
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
