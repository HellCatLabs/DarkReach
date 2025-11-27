pub mod ws;

use serde::{Deserialize, Serialize};

/// Message envoyé par l’agent au C2 après connexion
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentHello {
    pub hostname: String,
    pub os: String,
}

/// Tâche envoyée par le C2 à l’agent
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: String,
    pub command: String,
}

/// Résultat envoyé par l’agent après exécution d’une tâche
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentTaskResult {
    pub id: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
