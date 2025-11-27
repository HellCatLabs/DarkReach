use std::collections::HashMap;
use crate::api::agents::AgentInfo;
use crate::api::ops::OperationInfo;
use crate::api::tasks::TaskInfo;

#[derive(Clone)]
pub struct C2State {
    pub agents: HashMap<String, AgentInfo>,
    pub operations: HashMap<String, OperationInfo>,
    pub tasks: HashMap<String, TaskInfo>,
}


impl C2State {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            operations: HashMap::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

}

impl Default for C2State {
    fn default() -> Self {
        Self::new()
    }
}