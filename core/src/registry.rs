use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

// 1. Define errors that can happen here
#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Could not find home directory")]
    HomeDirNotFound,
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization Error: {0}")]
    Serde(#[from] serde_json::Error),
}

// 2. Define the "Identity Card" (The JSON structure)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppRegistration {
    pub app_id: String,
    pub name: String,
    pub socket_path: String,
    pub pid: u32,
}

impl AppRegistration {
    // A helper to create a new registration
    pub fn new(app_id: &str, name: &str, socket_path: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            name: name.to_string(),
            socket_path: socket_path.to_string(),
            pid: std::process::id(), // Automatically get current Process ID
        }
    }

    // 3. The function to save this card to disk
    pub fn register(&self) -> Result<(), RegistryError> {
        // Find ~/.proprio/registry/
        let mut path = dirs::home_dir().ok_or(RegistryError::HomeDirNotFound)?;
        path.push(".proprio");
        path.push("registry");

        // Create the directory if it doesn't exist
        fs::create_dir_all(&path)?;

        // Save as <app_id>.json
        path.push(format!("{}.json", self.app_id));
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;

        println!("Registered app: {}", self.app_id);
        Ok(())
    }
}

// 1. Define a helper struct for listing apps
pub struct Registry;

impl Registry {
    /// Scans the ~/.proprio/registry/ folder and returns all active apps
    pub fn list() -> Result<Vec<AppRegistration>, RegistryError> {
        let mut apps = Vec::new();
        
        // Find the path: ~/.proprio/registry
        let mut path = dirs::home_dir().ok_or(RegistryError::HomeDirNotFound)?;
        path.push(".proprio");
        path.push("registry");

        // If folder doesn't exist, just return empty list
        if !path.exists() {
            return Ok(apps);
        }

        // Read every file in the directory
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            // Only parse if it is a .json file
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Read the file content
                let content = fs::read_to_string(&path)?;
                
                // Parse the JSON back into our Struct
                // If it fails (corrupt file), we just skip it
                if let Ok(app) = serde_json::from_str::<AppRegistration>(&content) {
                    apps.push(app);
                }
            }
        }
        
        Ok(apps)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // This will hold the JSON Schema
}

impl Tool {
    pub fn new(name: &str, description: &str, params: serde_json::Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters: params,
        }
    }
}