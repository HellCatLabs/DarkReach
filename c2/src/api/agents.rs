use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::state::C2State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRegisterRequest {
    pub hostname: String,
    pub os: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub hostname: String,
    pub os: String,
    pub last_seen: i64,
}

/// POST /api/agents/register
/// Called by the agent when connecting
#[post("/agents/register")]
pub async fn register_agent(
    state: web::Data<Arc<RwLock<C2State>>>,
    req: web::Json<AgentRegisterRequest>,
) -> HttpResponse {
    let mut state = state.write().await;

    // generate an ID
    let id = Uuid::new_v4().to_string();

    let agent = AgentInfo {
        id: id.clone(),
        hostname: req.hostname.clone(),
        os: req.os.clone(),
        last_seen: chrono::Utc::now().timestamp(),
    };

    state.agents.insert(id.clone(), agent.clone());

    HttpResponse::Ok().json(agent)
}

/// GET /api/agents
/// Called by the CLI to list agents
#[get("/agents")]
pub async fn list_agents(
    state: web::Data<Arc<RwLock<C2State>>>,
) -> HttpResponse {
    let state = state.read().await;
    let agents: Vec<_> = state.agents.values().cloned().collect();
    HttpResponse::Ok().json(agents)
}

/// Configure routes for app factory
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register_agent);
    cfg.service(list_agents);
}