use crate::macros::*;

use teaql_tool_std::diff::DiffTool;

define_context_facade!("std", diff, ContextDiffExt, ContextDiffFacade);

#[cfg(feature = "std")]
impl<'a> ContextDiffFacade<'a> {
    delegate_comment! { DiffTool::new(),
        fn text_diff(&self, old: &str, new: &str) -> String
    }
}
