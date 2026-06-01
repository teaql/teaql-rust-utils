use teaql_tool_core::{Result, TeaQLToolError};
use barcoders::sym::code128::Code128;
use barcoders::generators::image::Image;

#[derive(Debug, Clone)]
pub struct BarcodeTool;

impl BarcodeTool {
    pub fn new() -> Self { Self }

    pub fn generate_code128_png(&self, data: &str, output_path: &str) -> Result<()> {
        let barcode = Code128::new(data).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let png = Image::png(80);
        let bytes = png.generate(&barcode.encode()[..]).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        std::fs::write(output_path, bytes).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(())
    }

    pub fn generate_code128_svg(&self, data: &str) -> Result<String> {
        let barcode = Code128::new(data).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let svg = barcoders::generators::svg::SVG::new(80);
        let data = svg.generate(&barcode.encode()[..]).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(data)
    }
}

impl Default for BarcodeTool {
    fn default() -> Self { Self::new() }
}
