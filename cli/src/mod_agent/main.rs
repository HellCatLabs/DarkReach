use clap::{Args, Subcommand};

#[derive(Args)]
pub struct AgentCommand {
    #[command(subcommand)]
    pub action: AgentAction,
}

#[derive(Subcommand)]
pub enum AgentAction {
    /// List all available agents
    List {},

    /// Interact with a specific agent
    Interact {},

    /// Add a new agent
    Add {},

    /// Remove an existing agent
    Remove {},
}

impl AgentCommand {
    pub fn run(self) {
        match self.action {
            AgentAction::List { } => {
                println!("TODO, list all available agents");
            }
            AgentAction::Interact { } => {
                println!("TODO, interact with a specific agent");
            }
            AgentAction::Add { } => {
                println!("TODO, add a new agent");
            }
            AgentAction::Remove { } => {
                println!("TODO, remove an existing agent");
            }
        }
    }
}