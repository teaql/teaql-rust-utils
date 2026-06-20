use crate::macros::*;

use teaql_tool_std::hash::HashTool;

define_context_facade!("std", hash, ContextHashExt, ContextHashFacade);

#[cfg(feature = "std")]
impl<'a> ContextHashFacade<'a> {
    delegate_comment! { HashTool::new(),
        fn sha256(&self, data: &[u8]) -> String;
        fn sha512(&self, data: &[u8]) -> String;
        fn blake3(&self, data: &[u8]) -> String;
        fn crc32(&self, data: &[u8]) -> u32
    }
}
