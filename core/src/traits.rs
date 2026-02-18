use serde_json::Value;

// This is the contract every App must sign
pub trait ProprioHandler: Send + Sync {
    fn execute(&self, method: &str, params: Option<Value>) -> Result<Value, String>;
}