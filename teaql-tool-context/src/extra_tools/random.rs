use crate::macros::*;

use teaql_tool_extra::random::RandomTool;

define_context_facade!("extra", random, ContextRandomExt, ContextRandomFacade);

#[cfg(feature = "extra")]
impl<'a> ContextRandomFacade<'a> {
    delegate_comment! { RandomTool::new(),
        fn int(&self, min: i64, max: i64) -> i64;
        fn float(&self, min: f64, max: f64) -> f64;
        fn boolean(&self) -> bool
    }
}
