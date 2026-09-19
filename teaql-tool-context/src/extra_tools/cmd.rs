use crate::macros::*;

use teaql_tool_core::{MustAuditAs, Result};
use teaql_tool_extra::cmd::CmdTool;

define_context_facade!("extra", cmd, ContextCmdExt, ContextCmdFacade);

#[cfg(feature = "extra")]
impl<'a> ContextCmdFacade<'a> {
    pub fn run_with_timeout(
        &self,
        cmd_line: &str,
        timeout_secs: u64,
    ) -> MustAuditAs<Result<(String, String, i32)>> {
        let cmd_line = cmd_line.to_owned();
        MustAuditAs::new(move |_desc| CmdTool::new().run_with_timeout(&cmd_line, timeout_secs))
    }
}
