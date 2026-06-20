use crate::macros::*;

use teaql_tool_std::id::IdTool;
use teaql_tool_core::MustPurpose;

define_context_facade!("std", id, ContextIdExt, ContextIdFacade);

#[cfg(feature = "std")]
impl<'a> ContextIdFacade<'a> {
    delegate_comment! { IdTool::new(),
        fn uuid(&self) -> String;
        fn uuid_v7(&self) -> String;
        fn ulid(&self) -> String;
        fn nanoid(&self) -> String;
        fn with_prefix(&self, prefix: &str) -> String
    }
}
