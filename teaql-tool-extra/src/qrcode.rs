use teaql_tool_core::{Result, TeaQLToolError};
use qrcode::QrCode;
use image::Luma;

#[derive(Debug, Clone)]
pub struct QrcodeTool;

impl QrcodeTool {
    pub fn new() -> Self { Self }

    pub fn generate_png(&self, data: &str, output_path: &str) -> Result<()> {
        let code = QrCode::new(data).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let image = code.render::<Luma<u8>>().build();
        image.save(output_path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(())
    }

    pub fn generate_svg(&self, data: &str) -> Result<String> {
        let code = QrCode::new(data).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let svg = code.render()
            .min_dimensions(200, 200)
            .dark_color(qrcode::render::svg::Color("#000000"))
            .light_color(qrcode::render::svg::Color("#ffffff"))
            .build();
        Ok(svg)
    }
}

impl Default for QrcodeTool {
    fn default() -> Self { Self::new() }
}
