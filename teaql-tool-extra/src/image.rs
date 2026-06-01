use teaql_tool_core::{Result, TeaQLToolError};
use image::imageops::FilterType;

#[derive(Debug, Clone)]
pub struct ImageTool;

impl ImageTool {
    pub fn new() -> Self { Self }

    pub fn resize(&self, input_path: &str, output_path: &str, width: u32, height: u32) -> Result<()> {
        let img = image::open(input_path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let resized = img.resize(width, height, FilterType::Lanczos3);
        resized.save(output_path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(())
    }
}

impl Default for ImageTool {
    fn default() -> Self { Self::new() }
}
