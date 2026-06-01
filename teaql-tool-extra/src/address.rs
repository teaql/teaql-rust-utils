use teaql_tool_core::Result;

#[derive(Debug, Clone)]
pub struct AddressTool;

impl AddressTool {
    pub fn new() -> Self { Self }

    /// A simple mock for address parsing (can be expanded later)
    pub fn extract_province(&self, address: &str) -> String {
        if let Some(idx) = address.find("省") {
            address[0..idx+3].to_string()
        } else {
            String::new()
        }
    }
}

impl Default for AddressTool {
    fn default() -> Self { Self::new() }
}
