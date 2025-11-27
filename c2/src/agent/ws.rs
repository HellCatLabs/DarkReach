use actix_web::{get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use crate::state::C2State;

use crate::ws::agent_session::AgentSession;
use std::sync::Arc;
use tokio::sync::RwLock;

#[get("/ws/agent")]
pub async fn ws_agent(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Arc<RwLock<C2State>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let session = AgentSession::new(state.get_ref().clone());
    ws::start(session, &req, stream)
}