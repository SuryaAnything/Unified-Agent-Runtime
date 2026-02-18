use proprio_core::registry::{AppRegistration, Tool};
use proprio_core::server::IpcServer;
use proprio_core::router::Router;
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[*] Starting Dummy App (IPC Server)...");
    let socket_path = "/tmp/dummy.sock";
    
    // 1. Create the Router
    let mut router = Router::new();

    // --- TOOL 1: Draw Rectangle ---
    router.add("draw_rectangle", |params| {
        let p = params.unwrap_or(json!({}));
        // Handle float inputs from AI by casting to i64
        let width = p["width"].as_f64().unwrap_or(0.0) as i64;
        let height = p["height"].as_f64().unwrap_or(0.0) as i64;

        println!("[CMD] draw_rectangle called with {}x{}", width, height);
        
        Ok(json!({ 
            "status": "success", 
            "area": width * height,
            "timestamp": "2026-02-18T12:00:00Z"
        }))
    });

    // --- TOOL 2: Get Screen Size ---
    router.add("get_screen_size", |_| {
        println!("[CMD] get_screen_size called");
        Ok(json!({
            "width": 1920,
            "height": 1080,
            "unit": "pixels"
        }))
    });

    // 2. Define the Manifest
    let my_tools = vec![
        Tool::new(
            "draw_rectangle", 
            "Draws a rectangle on the screen.", 
            json!({ "width": "int", "height": "int" })
        ),
        Tool::new(
            "get_screen_size", 
            "Returns the current screen width and height.", 
            json!({}) 
        )
    ];

    // 3. Register Identity
    AppRegistration::new("com.test.dummy", "Dummy App", socket_path).register()?;

    // 4. Start Server
    println!("[+] Server ready.");
    println!("[+] Listening on {}...", socket_path);
    
    let server = IpcServer::new(socket_path, my_tools, router);
    server.run().await?;

    Ok(())
}