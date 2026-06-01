use dotenvy::dotenv;
use std::env;
use teaql_tool_core::{Result, TeaQLToolError};

pub struct ConfigTool;

impl ConfigTool {
    pub fn new() -> Self {
        Self
    }

    pub fn load_env(&self) -> Result<()> {
        let _ = dotenv(); // Ignore error if .env doesn't exist
        Ok(())
    }
    
    pub fn get_env(&self, key: &str) -> Result<String> {
        env::var(key).map_err(|_| TeaQLToolError::ExecutionError(format!("Env var {} not found", key)))
    }
}

impl Default for ConfigTool {
    fn default() -> Self {
        Self::new()
    }
}
