use crate::macros::*;

use teaql_tool_extra::cmd::CmdTool;
use teaql_tool_core::Result;

define_context_facade!("extra", cmd, ContextCmdExt, ContextCmdFacade);

#[cfg(feature = "extra")]
impl<'a> ContextCmdFacade<'a> {
    delegate_res_audit! { CmdTool::new(),
        fn run_with_timeout(&self, cmd_line: &str, timeout_secs: u64) -> (String, String, i32)
    }
}
