#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::convert::TryFrom;
use std::path;

use actix::prelude::*;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Service;
use actix_web::http::header::{CACHE_CONTROL, CacheControl, CacheDirective};
use actix_web::http::HeaderValue;

use crate::rooms::OnitamaServer;
use crate::routes::{create_room, event_receive, join_room, ServerData};

mod rooms;
mod messages;
mod routes;
mod utils;
#[cfg(feature = "agent")]
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
    info!("Does build path exist ({}): {}", built_path.as_os_str().to_string_lossy(), built_path.exists());
    info!("Starting server");
    HttpServer::new(move || {
        cfg_if::cfg_if! {
            if #[cfg(feature = "agent")] {
                use crate::routes::ai_room;
                let factory =
                    web::scope("/ws")
                        .route("/event", web::post().to(event_receive))
                        .route("/ai/{difficulty}", web::get().to(ai_room))
                        .route("/{key}", web::get().to(join_room))
                        .route("/", web::get().to(create_room));
            } else {
                let factory =
                    web::scope("/ws")
                        .route("/event", web::post().to(event_receive))
                        .route("/{key}", web::get().to(join_room))
                        .route("/", web::get().to(create_room));
            }
        }
        let app = App::new()
            // Cache all requests to paths in /static otherwise don't cache
            .wrap_fn(|req, srv| {
                let is_static = req.path().starts_with("/static")
                        || req.path().ends_with(".wasm");
                let is_serviceworker = req.path() == "/service-worker.js";
                let cache_static = match (is_static, is_serviceworker) {
                    (true, _) => CacheControl(vec![CacheDirective::MaxAge(86400), CacheDirective::Public, CacheDirective::Extension("immutable".to_string(), None)]).to_string(),
                    (false, false) => CacheControl(vec![
                        CacheDirective::Extension("s-maxage".to_owned(), Some("300".to_owned())),
                    ]).to_string(),
                    (false, true) => CacheControl(vec![
                        CacheDirective::Extension("stale-if-error".to_owned(), Some("86400".to_owned())),
                        CacheDirective::Extension("must-revalidate".to_owned(), None),
                    ]).to_string(),
                };
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    let cache_control: HeaderValue = HeaderValue::try_from(cache_static).expect("Oops");
                    res.headers_mut().insert(
                        CACHE_CONTROL, cache_control,
                    );
                    Ok(res)
                }
            })
            .app_data(data.clone())
            .service(factory);
        match built_path.exists() {
            true => app
                .service(
                    Files::new("/", built_path)
                        .index_file("index.html")
                ),
            false => app,
        }
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
