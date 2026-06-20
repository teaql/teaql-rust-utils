use crate::macros::*;

use teaql_tool_std::tree::TreeTool;
use serde_json::Value;
use teaql_tool_core::Result;

define_context_facade!("std", tree, ContextTreeExt, ContextTreeFacade);

#[cfg(feature = "std")]
impl<'a> ContextTreeFacade<'a> {
    delegate_res_comment! { TreeTool::new(),
        fn build(&self, flat_array: Value, id_field: &str, parent_id_field: &str, children_field: &str, root_parent_id: Value) -> Value
    }
}
