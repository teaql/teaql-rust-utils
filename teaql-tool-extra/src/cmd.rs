use teaql_tool_core::{Result, TeaQLToolError};
use std::process::Command;
use wait_timeout::ChildExt;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CmdTool;

impl CmdTool {
    pub fn new() -> Self { Self }

    pub fn run_with_timeout(&self, cmd_line: &str, timeout_secs: u64) -> Result<(String, String, i32)> {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() {
            return Err(TeaQLToolError::ExecutionError("Empty command".into()));
        }
        
        let mut cmd = Command::new(parts[0]);
        cmd.args(&parts[1..]);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        
        let mut child = cmd.spawn().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let timeout = Duration::from_secs(timeout_secs);
        
        match child.wait_timeout(timeout).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))? {
            Some(status) => {
                let out = child.wait_with_output().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Ok((stdout, stderr, status.code().unwrap_or(-1)))
            }
            None => {
                let _ = child.kill();
                let _ = child.wait();
                Err(TeaQLToolError::ExecutionError("Command timed out".into()))
            }
        }
    }
}

impl Default for CmdTool {
    fn default() -> Self { Self::new() }
}
