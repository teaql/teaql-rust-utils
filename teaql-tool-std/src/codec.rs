use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use hex;
use urlencoding;
use html_escape;
use teaql_tool_core::{Result, TeaQLToolError};

pub struct CodecTool;

impl CodecTool {
    pub fn new() -> Self {
        Self
    }

    pub fn base64_encode<T: AsRef<[u8]>>(&self, data: T) -> String {
        BASE64.encode(data)
    }

    pub fn base64_decode(&self, data: &str) -> Result<Vec<u8>> {
        BASE64.decode(data).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn hex_encode<T: AsRef<[u8]>>(&self, data: T) -> String {
        hex::encode(data)
    }

    pub fn hex_decode(&self, data: &str) -> Result<Vec<u8>> {
        hex::decode(data).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn url_encode(&self, data: &str) -> String {
        urlencoding::encode(data).into_owned()
    }

    pub fn url_decode(&self, data: &str) -> Result<String> {
        urlencoding::decode(data)
            .map(|cow| cow.into_owned())
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn html_escape(&self, data: &str) -> String {
        html_escape::encode_text(data).into_owned()
    }

    pub fn html_unescape(&self, data: &str) -> String {
        html_escape::decode_html_entities(data).into_owned()
    }
}

impl Default for CodecTool {
    fn default() -> Self {
        Self::new()
    }
}
