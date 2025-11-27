extern crate darkreach_cli;

use clap::{Parser, Subcommand};

mod mod_c2;
mod mod_agent;

#[derive(Parser)]
#[command(name = "darkreach")]
#[command(about = "DarkReach CLI controller", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage the C2 server
    C2(mod_c2::main::C2Command),
    /// Manage agents
    Agent(mod_agent::main::AgentCommand),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::C2(cmd) => cmd.run(),
        Commands::Agent(cmd) => cmd.run(),
    }
}