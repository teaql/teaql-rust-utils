use crate::macros::*;

use teaql_tool_extra::kv::KvTool;
use teaql_tool_core::Result;

define_context_facade!("extra", kv, ContextKvExt, ContextKvFacade);

#[cfg(feature = "extra")]
impl<'a> ContextKvFacade<'a> {
    delegate_res_purpose! { KvTool::new(),
        fn open(&self, path: &str) -> sled::Db
    }
}
