use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::C2State;

/// Request pour démarrer une opération
#[derive(Debug, Serialize, Deserialize)]
pub struct StartOpRequest {
    pub name: String,
    pub target_agent: String,
}

/// Info sur une opération
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationInfo {
    pub id: String,
    pub name: String,
    pub target_agent: String,
    pub started_at: i64,
    pub status: String, // "running", "finished", "error", etc.
}

/// POST /api/ops/start
#[post("/ops/start")]
pub async fn start_operation(
    state: web::Data<Arc<RwLock<C2State>>>,
    req: web::Json<StartOpRequest>,
) -> HttpResponse {
    let mut state = state.write().await;

    // Validation simple
    if !state.agents.contains_key(&req.target_agent) {
        return HttpResponse::BadRequest().json(
            serde_json::json!({ "error": "Unknown agent" }),
        );
    }

    // ID unique
    let id = uuid::Uuid::new_v4().to_string();

    let op = OperationInfo {
        id: id.clone(),
        name: req.name.clone(),
        target_agent: req.target_agent.clone(),
        started_at: chrono::Utc::now().timestamp(),
        status: "running".to_string(),
    };

    state.operations.insert(id.clone(), op.clone());

    HttpResponse::Ok().json(op)
}

/// GET /api/ops
#[get("/ops")]
pub async fn list_operations(
    state: web::Data<Arc<RwLock<C2State>>>,
) -> HttpResponse {
    let state = state.read().await;
    let ops: Vec<_> = state.operations.values().cloned().collect();
    HttpResponse::Ok().json(ops)
}

/// GET /api/ops/{id}
#[get("/ops/{id}")]
pub async fn get_operation(
    state: web::Data<Arc<RwLock<C2State>>>,
    path: web::Path<String>,
) -> HttpResponse {
    let state = state.read().await;
    let id = path.into_inner();

    if let Some(op) = state.operations.get(&id) {
        HttpResponse::Ok().json(op)
    } else {
        HttpResponse::NotFound().json(
            serde_json::json!({ "error": "Operation not found" }),
        )
    }
}

/// Enregistrement des routes dans l'app
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(start_operation);
    cfg.service(list_operations);
    cfg.service(get_operation);
}