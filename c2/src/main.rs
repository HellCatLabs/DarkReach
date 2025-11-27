use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tokio::sync::RwLock;

mod api;
mod agent;
mod state;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Shared global state wrapped correctly
    let shared_state = Arc::new(RwLock::new(state::C2State::default()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_state.clone()))
            .configure(api::agents::configure)
            .configure(api::ops::configure)
            .configure(api::tasks::configure)
            .service(agent::ws::ws_agent) // Ta route WebSocket
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}