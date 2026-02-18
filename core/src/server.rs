use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use std::fs;
use std::sync::Arc;
use crate::protocol::{JsonRpcRequest, JsonRpcResponse};
use crate::registry::Tool;
use crate::traits::ProprioHandler; // <--- Import the new Interface
use serde_json::json;

pub struct IpcServer {
    socket_path: String,
    tools: Arc<Vec<Tool>>,
    handler: Arc<dyn ProprioHandler>, // <--- The "Plug" for the app logic
}

impl IpcServer {
    // Now we accept a "handler" (the app logic)
    pub fn new(socket_path: &str, tools: Vec<Tool>, handler: impl ProprioHandler + 'static) -> Self {
        Self {
            socket_path: socket_path.to_string(),
            tools: Arc::new(tools),
            handler: Arc::new(handler),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        if Path::new(&self.socket_path).exists() {
            fs::remove_file(&self.socket_path)?;
        }

        let listener = UnixListener::bind(&self.socket_path)?;
        println!("👂 Listening on {}", self.socket_path);

        loop {
            let (socket, _) = listener.accept().await?;
            let tools_ref = self.tools.clone();
            let handler_ref = self.handler.clone(); // Share the handler

            tokio::spawn(async move {
                if let Err(e) = handle_client(socket, tools_ref, handler_ref).await {
                    println!("Client error: {}", e);
                }
            });
        }
    }
}

async fn handle_client(
    mut socket: UnixStream, 
    tools: Arc<Vec<Tool>>, 
    handler: Arc<dyn ProprioHandler>
) -> std::io::Result<()> {
    let mut buffer = [0; 4096];

    // --- START LOOP (Keep connection alive) ---
    loop {
        // 1. Wait for data
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return Ok(()), // Client closed connection (Normal exit)
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("📩 Received: {}", message);

        // 2. Process Request
        let response = match serde_json::from_str::<JsonRpcRequest>(&message) {
            Ok(req) => {
                if req.method == "__proprio_ping__" {
                    JsonRpcResponse::ok(req.id, json!("pong"))
                } 
                else if req.method == "__proprio_manifest__" {
                    JsonRpcResponse::ok(req.id, json!({ "tools": *tools }))
                } 
                else {
                    match handler.execute(&req.method, req.params) {
                        Ok(result) => JsonRpcResponse::ok(req.id, result),
                        Err(e) => JsonRpcResponse::err(req.id, -32000, &e),
                    }
                }
            },
            Err(_) => JsonRpcResponse::err(None, -32700, "Parse error")
        };

        // 3. Send Response
        let response_str = serde_json::to_string(&response)?;
        socket.write_all(response_str.as_bytes()).await?;
        
        // Loop goes back to top to wait for next message!
    }
}