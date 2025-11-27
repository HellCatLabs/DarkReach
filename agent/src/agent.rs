use crate::proto::*;
use tokio_tungstenite::connect_async;
use futures::{SinkExt, StreamExt};
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let url = "ws://127.0.0.1:8080/ws/agent";

    println!("[agent] Connecting to {}", url);

    let (mut ws, _) = connect_async(url).await?;
    println!("[agent] Connected!");

    // Send hello message
    let hello = HelloMsg {
        msg_type: "hello".into(),
        hostname: hostname::get()?.to_string_lossy().to_string(),
        os: std::env::consts::OS.to_string(),
    };

    ws.send(tungstenite::Message::Text(serde_json::to_string(&hello)?)).await?;

    // Main loop
    while let Some(msg) = ws.next().await {
        let msg = msg?;

        match msg {
            tungstenite::Message::Text(text) => {
                println!("[agent] Received: {}", text);

                // Parse task
                let task: TaskMsg = match serde_json::from_str(&text) {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                // Execute command
                let output = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(&task.command)
                    .output()
                    .await?;

                let res = ResultMsg {
                    msg_type: "result".into(),
                    id: task.id,
                    stdout: String::from_utf8_lossy(&output.stdout).into(),
                    stderr: String::from_utf8_lossy(&output.stderr).into(),
                    exit_code: output.status.code().unwrap_or(-1),
                };

                // Send result
                ws.send(tungstenite::Message::Text(serde_json::to_string(&res)?)).await?;
            }

            tungstenite::Message::Ping(p) => {
                ws.send(tungstenite::Message::Pong(p)).await?;
            }

            tungstenite::Message::Close(_) => {
                println!("[agent] Server closed connection");
                break;
            }

            _ => {}
        }
    }

    Ok(())
}