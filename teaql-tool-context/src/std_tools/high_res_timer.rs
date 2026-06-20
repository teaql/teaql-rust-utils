use crate::macros::*;

use teaql_tool_std::high_res_timer::{HighResTimerTool, HighResTimer};

define_context_facade!("std", high_res_timer, ContextHighResTimerExt, ContextHighResTimerFacade);

#[cfg(feature = "std")]
impl<'a> ContextHighResTimerFacade<'a> {
    delegate_comment! { HighResTimerTool::new(),
        fn start(&self) -> HighResTimer
    }
}
