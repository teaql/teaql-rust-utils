use crate::macros::*;

use teaql_tool_std::codec::CodecTool;
use teaql_tool_core::Result;

define_context_facade!("std", codec, ContextCodecExt, ContextCodecFacade);

#[cfg(feature = "std")]
impl<'a> ContextCodecFacade<'a> {
    delegate_comment! { CodecTool::new(),
        fn url_encode(&self, data: &str) -> String;
        fn html_escape(&self, data: &str) -> String;
        fn html_unescape(&self, data: &str) -> String
    }
    delegate_res_comment! { CodecTool::new(),
        fn base64_decode(&self, data: &str) -> Vec<u8>;
        fn hex_decode(&self, data: &str) -> Vec<u8>;
        fn url_decode(&self, data: &str) -> String
    }

    pub fn base64_encode<T: AsRef<[u8]>>(&self, data: T) -> String {
        CodecTool::new().base64_encode(data)
    }

    pub fn hex_encode<T: AsRef<[u8]>>(&self, data: T) -> String {
        CodecTool::new().hex_encode(data)
    }
}
