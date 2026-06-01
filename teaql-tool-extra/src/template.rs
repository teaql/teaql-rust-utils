use tera::{Tera, Context};
use teaql_tool_core::{Result, TeaQLToolError};

#[derive(Debug, Clone)]
pub struct TemplateTool;

impl TemplateTool {
    pub fn new() -> Self { Self }

    pub fn render(&self, template: &str, ctx_json: &str) -> Result<String> {
        let mut context = Context::new();
        let value: serde_json::Value = serde_json::from_str(ctx_json).map_err(|e| TeaQLToolError::ParseError(e.to_string()))?;
        if let serde_json::Value::Object(map) = value {
            for (k, v) in map {
                context.insert(k, &v);
            }
        }
        Tera::one_off(template, &context, true).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
    }
}

impl Default for TemplateTool {
    fn default() -> Self { Self::new() }
}
