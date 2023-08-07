use std::str::FromStr;

use crate::rooms::{OnitamaServer, RoomWs};
use crate::utils::{get_identifier, get_useragent, get_ip};
use actix::prelude::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::StreamExt;
use onitamalib::GameEvent;
use serde_cbor::de;
use uuid::Uuid;

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

pub struct ServerData {
    pub logger: slog::Logger,
    pub server_addr: Addr<OnitamaServer>,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "agent")] {
        use onitamalib::AiAgent;

        use crate::agents::AgentWs;
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
    }
}

pub async fn event_receive(
    req: HttpRequest,
    mut body: web::Payload,
    server_data: web::Data<ServerData>,
) -> Result<String, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }
    let data: GameEvent = match de::from_slice(bytes.as_ref()) {
        Ok(data) => data,
        Err(err) => {
            slog::warn!(
                server_data.logger,
                "Failed to deserialize event: {:?}",
                &err
            );
            return Ok("ok".to_string());
        }
    };

    match data {
        GameEvent::Start {
            against,
            training,
            meta,
        } => {
            slog::info!(server_data.logger,
                "Game started against {}", &against;
                "opponent" => &against,
                "event" => "started",
                "uid" => meta.uid,
                "training" => training,
                "build" => meta.build,
                "user_agent" => get_useragent(&req),
                "ip" => get_ip(&req),
            );
        }
        GameEvent::End {
            against,
            winner,
            training,
            meta,
        } => {
            slog::info!(
                server_data.logger,
                "Game ended against {} winner was {}", &against, winner;
                "opponent" => &against,
                "event" => "ended",
                "uid" => meta.uid,
                "training" => training,
                "build" => meta.build,
                "user_agent" => get_useragent(&req),
                "ip" => get_ip(&req),
            );
        }
    };
    Ok("test".to_string())
}
