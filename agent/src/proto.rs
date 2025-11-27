use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloMsg {
    pub msg_type: String,
    pub hostname: String,
    pub os: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMsg {
    pub id: String,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultMsg {
    pub msg_type: String,
    pub id: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}