use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use crate::traits::ProprioHandler;

// 1. Define what a "Handler Function" looks like
// It takes JSON params and returns a JSON Result
type HandlerFn = Box<dyn Fn(Option<Value>) -> Result<Value, String> + Send + Sync>;

// 2. The Router Struct
pub struct Router {
    routes: HashMap<String, HandlerFn>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    // 3. The method to register a new tool
    pub fn add<F>(&mut self, name: &str, func: F)
    where
        F: Fn(Option<Value>) -> Result<Value, String> + Send + Sync + 'static,
    {
        self.routes.insert(name.to_string(), Box::new(func));
    }
}

// 4. Implement the Trait so the Server can use this Router
impl ProprioHandler for Router {
    fn execute(&self, method: &str, params: Option<Value>) -> Result<Value, String> {
        // Look up the function in our HashMap
        if let Some(func) = self.routes.get(method) {
            // Run it!
            func(params)
        } else {
            Err(format!("Method '{}' not found in Router", method))
        }
    }
}