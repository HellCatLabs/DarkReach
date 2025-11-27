use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::C2State;

/// Durée max d’inactivité avant fermeture WS
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct AgentSession {
    pub hb: Instant,
    pub state: Arc<RwLock<C2State>>,
    pub agent_id: Option<String>,
}

impl AgentSession {
    pub fn new(state: Arc<RwLock<C2State>>) -> Self {
        Self {
            hb: Instant::now(),
            state,
            agent_id: None,
        }
    }

    /// ping régulier (heartbeat)
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(10), |act, ctx| {
            if Instant::now().duration_since(act.hb) > HEARTBEAT_TIMEOUT {
                println!("Agent heartbeat timeout, closing socket.");
                ctx.stop();
                return;
            }
            ctx.ping(b"ping");
        });
    }
}

/// Obligatoire : transformer la session en Actor
impl Actor for AgentSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        println!("New agent connected via WebSocket");
    }
}

/// Handler des messages WebSocket entrants
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AgentSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }

            Ok(ws::Message::Text(text)) => {
                println!("Received from agent: {}", text);

                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    if json["msg_type"] == "hello" {
                        let hostname = json["hostname"].as_str().unwrap_or("unknown").to_string();
                        let os = json["os"].as_str().unwrap_or("unknown").to_string();

                        let id = uuid::Uuid::new_v4().to_string();

                        self.agent_id = Some(id.clone());

                        let agent = crate::api::agents::AgentInfo {
                            id: id.clone(),
                            hostname,
                            os,
                            last_seen: chrono::Utc::now().timestamp(),
                        };

                        {
                            let state = self.state.clone();
                            tokio::spawn(async move {
                                let mut state = state.write().await;
                                state.agents.insert(id.clone(), agent);
                                println!("Registered new agent {}", id);
                            });
                        }
                        
                    }
                }
            }

            Ok(ws::Message::Close(_)) => {
                println!("Agent disconnected");
                if let Some(agent_id) = self.agent_id.clone() {
                    let state = self.state.clone();
                    tokio::spawn(async move {
                        let mut state = state.write().await;
                        state.agents.remove(&agent_id);
                        println!("Removed agent {} from state", agent_id);
                    });
                }
                ctx.stop();
            }

            _ => {}
        }
    }
}