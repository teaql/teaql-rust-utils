use teaql_tool_core::{Result, TeaQLToolError};

#[derive(Debug, Clone)]
pub struct KvTool;

impl KvTool {
    pub fn new() -> Self { Self }

    pub fn open(&self, path: &str) -> Result<sled::Db> {
        sled::open(path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
    }
}

impl Default for KvTool {
    fn default() -> Self { Self::new() }
}
