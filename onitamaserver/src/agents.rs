use std::time::Duration;

use actix::{Actor, Addr, StreamHandler, Handler, SyncContext, SyncArbiter, AsyncContext};
use actix_web_actors::ws;
use serde_cbor::ser;

use onitamalib::{GameMessage, GameState, Player, AiAgent};

use crate::messages::{AgentRequest, AgentResponse};


pub struct Agent {
    state: GameState,
    id: String,
    ai: AiAgent,
}

impl Agent {
    pub fn new(id: String, ai: AiAgent) -> Agent {
        let state = GameState::new();
        Agent { id, state, ai }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AgentException {
    IllegalMove,
    InvalidMessageForState,
    AgentError,
}

const TIMEOUT: Duration = Duration::from_millis(500);

impl Agent {
    fn play_move(&mut self, state: GameState) -> Result<GameMessage, AgentException> {
        // The state is guaranteed to be Playing
        let (game_move, expected_score) = match self.ai.play_move(&state, TIMEOUT) {
            None => {
                error!("No moves available");
                return Err(AgentException::AgentError);
            },
            Some((best_move, best_score)) => (best_move, best_score),
        };
        info!("Expected score: {}, move = {:?}", expected_score, &game_move);
        self.state = match state.try_move(game_move) {
            Ok(state) => state,
            Err(err) => {
                error!("Agent attempted invalid move: {}", err);
                return Err(AgentException::AgentError);
            },
        };
        let msg = GameMessage::Move { game_move };
        Ok(msg)
    }
    fn handle_game_message(&mut self, msg: GameMessage) -> Result<GameMessage, AgentException> {
        match (&self.state, msg) {
            (GameState::Finished { .. }, GameMessage::RequestRematch) => {
                info!("Starting rematch");
                self.state = GameState::new();
                Ok(GameMessage::Initialize {
                    state: self.state.clone(),
                    room_id: "ai".to_string(),
                    player: Player::Red,
                    waiting: false,
                })
            }
            (_, GameMessage::Joined) => {
                info!("Game started");
                Ok(GameMessage::Initialize {
                    state: self.state.clone(),
                    room_id: "ai".to_string(),
                    player: Player::Red,
                    waiting: false,
                })
            },
            (state, GameMessage::Move { game_move }) => {
                let state = match state.try_move(game_move) {
                    Ok(state) => state,
                    Err(err) => {
                        warn!("Invalid Move: {}", err);
                        return Err(AgentException::IllegalMove);
                    },
                };
                self.state = state;
                match state {
                    GameState::Finished { .. } => {
                        Ok(GameMessage::RequestRematch)
                    },
                    state => self.play_move(state),
                }
            },
            (state, msg) => {
                warn!("Unexpected transition: state={:?}, msg={:?}", state, msg);
                Err(AgentException::InvalidMessageForState)
            },
        }
    }
}

impl Actor for Agent {
    type Context = SyncContext<Self>;
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Agent stopped");
    }
}

impl Handler<AgentRequest> for Agent {
    type Result = ();

    fn handle(&mut self, msg: AgentRequest, _ctx: &mut Self::Context) -> Self::Result {
        let AgentRequest { msg, addr } = msg;
        let resp = self.handle_game_message(msg);
        match &self.state {
            GameState::Finished { winner, .. } => {
                info!("Game finished, {:?} won: {}", winner, self.id);
            },
            _ => {},
        };
        let resp = AgentResponse { resp };
        addr.do_send(resp);
    }
}

pub struct AgentWs {
    agent: Addr<Agent>,
}

impl AgentWs {
    pub fn new(id: String, ai: AiAgent) -> AgentWs {
        let agent = SyncArbiter::start(1, move || Agent::new(id.clone(), ai));
        AgentWs { agent }
    }
}

impl Actor for AgentWs {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Agent created");
        let msg = AgentRequest { msg: GameMessage::Joined, addr: ctx.address() };
        self.agent.do_send(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AgentWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let data = match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
                return;
            },
            Ok(ws::Message::Binary(data)) => data,
            msg => {
                warn!("Unexpected message type: {:?}", msg);
                return;
            },
        };
        let msg: GameMessage = match serde_cbor::from_slice(data.as_ref()) {
            Ok(msg) => msg,
            Err(err) => {
                warn!("Failed to deserialize game message: {}", err);
                return;
            },
        };
        let msg = AgentRequest { msg, addr: ctx.address() };
        match self.agent.try_send(msg) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to send msg to agent: {:?}", err);
                ctx.close(None);
            },
        };
    }
}

impl Handler<AgentResponse> for AgentWs {
    type Result = ();
    fn handle(&mut self, msg: AgentResponse, ctx: &mut Self::Context) {
        let msg = match msg.resp {
            Ok(msg) => msg,
            Err(err) => {
                let msg = format!("Error: {:?}", err);
                ctx.text(msg);
                return;
                // ctx.close(None);
            },
        };
        let msg = ser::to_vec(&msg).expect("failed to serialize message");
        ctx.binary(msg);
    }
}
