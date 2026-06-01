use teaql_tool_core::{Result, TeaQLToolError};
use arboard::Clipboard;

#[derive(Debug, Clone)]
pub struct ClipboardTool;

impl ClipboardTool {
    pub fn new() -> Self { Self }

    pub fn read_text(&self) -> Result<String> {
        let mut clipboard = Clipboard::new().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        clipboard.get_text().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
    }

    pub fn write_text(&self, text: &str) -> Result<()> {
        let mut clipboard = Clipboard::new().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        clipboard.set_text(text).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
    }
}

impl Default for ClipboardTool {
    fn default() -> Self { Self::new() }
}
