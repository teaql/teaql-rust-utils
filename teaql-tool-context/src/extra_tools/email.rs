use crate::macros::*;

use teaql_tool_extra::email::EmailTool;
use teaql_tool_core::Result;

define_context_facade!("extra", email, ContextEmailExt, ContextEmailFacade);

#[cfg(feature = "extra")]
impl<'a> ContextEmailFacade<'a> {
    delegate_res_audit! { EmailTool::new(),
        fn send(&self, server: &str, user: &str, pass: &str, from: &str, to: &str, subject: &str, body: &str) -> ()
    }
}
