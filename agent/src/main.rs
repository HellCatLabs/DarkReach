mod proto;
mod agent;

#[tokio::main]
async fn main() {
    if let Err(e) = agent::run().await {
        eprintln!("Agent error: {}", e);
    }
}