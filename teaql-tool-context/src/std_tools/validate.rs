use crate::macros::*;

use teaql_tool_std::validate::ValidateTool;

define_context_facade!("std", validate, ContextValidateExt, ContextValidateFacade);

#[cfg(feature = "std")]
impl<'a> ContextValidateFacade<'a> {
    delegate_comment! { ValidateTool::new(),
        fn email(&self, val: &str) -> bool;
        fn url(&self, val: &str) -> bool;
        fn min_length(&self, val: &str, min: usize) -> bool;
        fn max_length(&self, val: &str, max: usize) -> bool
    }
}
