use reqwest::blocking::Client;
use std::time::Duration;
use teaql_tool_core::{Result, TeaQLToolError};

pub struct HttpTool {
    client: Client,
}

impl HttpTool {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        Self { client }
    }

    pub fn get(&self, url: &str) -> Result<String> {
        self.client
            .get(url)
            .send()
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Http get failed: {}", e)))?
            .text()
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Http read text failed: {}", e)))
    }
}

impl Default for HttpTool {
    fn default() -> Self {
        Self::new()
    }
}
