use teaql_tool_core::{Result, TeaQLToolError};
use url::Url;
use percent_encoding::{utf8_percent_encode, percent_decode_str, NON_ALPHANUMERIC};

#[derive(Debug, Clone)]
pub struct UrlTool;

impl UrlTool {
    pub fn new() -> Self { Self }

    pub fn parse(&self, url: &str) -> Result<Url> {
        Url::parse(url).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn encode(&self, text: &str) -> String {
        utf8_percent_encode(text, NON_ALPHANUMERIC).to_string()
    }

    pub fn decode(&self, text: &str) -> Result<String> {
        percent_decode_str(text).decode_utf8().map(|s| s.into_owned()).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }
}

impl Default for UrlTool {
    fn default() -> Self { Self::new() }
}
