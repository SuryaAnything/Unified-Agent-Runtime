use serde::{Deserialize, Serialize};
use serde_json::Value;

// 1. The Request (What the AI sends)
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>, // "Value" can be anything (string, number, object)
    pub id: Option<u64>,
}

// 2. The Response (What the App replies)
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
    pub id: Option<u64>,
}

// 3. The Error (If something goes wrong)
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

// Helper to make a quick success response
impl JsonRpcResponse {
    pub fn ok(id: Option<u64>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }
    
    pub fn err(id: Option<u64>, code: i32, msg: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: msg.to_string(),
            }),
            id,
        }
    }
}