use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::C2State;

/// Task creation request (from CLI)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub operation_id: String,
    pub agent_id: String,
    pub command: String,
}

/// Information about a task stored in C2
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub operation_id: String,
    pub agent_id: String,
    pub command: String,
    pub created_at: i64,
    pub status: String, // queued, sent, done, error
}

/// POST /api/tasks/create
#[post("/tasks/create")]
pub async fn create_task(
    state: web::Data<Arc<RwLock<C2State>>>,
    req: web::Json<CreateTaskRequest>,
) -> HttpResponse {
    let mut state = state.write().await;

    // Validate agent exists
    if !state.agents.contains_key(&req.agent_id) {
        return HttpResponse::BadRequest().json(
            serde_json::json!({ "error": "Unknown agent" }),
        );
    }

    // Validate operation exists
    if !state.operations.contains_key(&req.operation_id) {
        return HttpResponse::BadRequest().json(
            serde_json::json!({ "error": "Unknown operation" }),
        );
    }

    // Generate ID
    let id = format!("task-{}", uuid::Uuid::new_v4().to_string());

    let task = TaskInfo {
        id: id.clone(),
        operation_id: req.operation_id.clone(),
        agent_id: req.agent_id.clone(),
        command: req.command.clone(),
        created_at: chrono::Utc::now().timestamp(),
        status: "queued".to_string(),
    };

    state.tasks.insert(id.clone(), task.clone());

    HttpResponse::Ok().json(task)
}

/// GET /api/tasks
#[get("/tasks")]
pub async fn list_tasks(
    state: web::Data<Arc<RwLock<C2State>>>,
) -> HttpResponse {
    let state = state.read().await;
    let tasks: Vec<_> = state.tasks.values().cloned().collect();
    HttpResponse::Ok().json(tasks)
}

/// GET /api/tasks/agent/{agent_id}
#[get("/tasks/agent/{agent_id}")]
pub async fn list_tasks_for_agent(
    state: web::Data<Arc<RwLock<C2State>>>,
    path: web::Path<String>,
) -> HttpResponse {
    let agent_id = path.into_inner();
    let state = state.read().await;

    let tasks: Vec<_> = state
        .tasks
        .values()
        .filter(|t| t.agent_id == agent_id)
        .cloned()
        .collect();

    HttpResponse::Ok().json(tasks)
}

/// GET /api/tasks/op/{op_id}
#[get("/tasks/op/{op_id}")]
pub async fn list_tasks_for_operation(
    state: web::Data<Arc<RwLock<C2State>>>,
    path: web::Path<String>,
) -> HttpResponse {
    let op_id = path.into_inner();
    let state = state.read().await;

    let tasks: Vec<_> = state
        .tasks
        .values()
        .filter(|t| t.operation_id == op_id)
        .cloned()
        .collect();

    HttpResponse::Ok().json(tasks)
}

/// Used by main app to mount task endpoints
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_task);
    cfg.service(list_tasks);
    cfg.service(list_tasks_for_agent);
    cfg.service(list_tasks_for_operation);
}