use clap::{Args, Subcommand};

#[derive(Args)]
pub struct C2Command {
    #[command(subcommand)]
    pub action: C2Action,
}

#[derive(Subcommand)]
pub enum C2Action {
    /// Install the C2 server on a remote machine
    Install {
        /// What host the C2 server should be listening ?
        #[arg(value_name = "HOST")]
        host: String,
    },

    /// Start the C2 server locally
    Start,

    /// Check C2 server status
    Status,
}

impl C2Command {
    pub fn run(self) {
        match self.action {
            C2Action::Install { host } => {
                println!("TODO, implement C2 installation");
                println!("Installing C2 on {}", host);
            }
            C2Action::Start => {
                println!("TODO, implement C2 start");
                println!("Starting DarkReach C2...");
            }
            C2Action::Status => {
                println!("TODO, implement C2 status check");
                println!("C2 status: OK");
            }
        }
    }
}