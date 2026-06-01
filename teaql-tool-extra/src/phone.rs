use teaql_tool_core::{Result, TeaQLToolError};

#[derive(Debug, Clone)]
pub struct PhoneTool;

impl PhoneTool {
    pub fn new() -> Self { Self }

    pub fn parse(&self, number: &str) -> Result<phonenumber::PhoneNumber> {
        phonenumber::parse(None, number).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn is_valid(&self, number: &str) -> bool {
        phonenumber::parse(None, number).map(|p| phonenumber::is_valid(&p)).unwrap_or(false)
    }

    pub fn format_international(&self, number: &str) -> Result<String> {
        self.parse(number).map(|p| p.format().mode(phonenumber::Mode::International).to_string())
    }
}

impl Default for PhoneTool {
    fn default() -> Self { Self::new() }
}
