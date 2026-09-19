use crate::macros::*;

use teaql_tool_core::{MustAuditAs, Result};
use teaql_tool_extra::email::EmailTool;

define_context_facade!("extra", email, ContextEmailExt, ContextEmailFacade);

#[cfg(feature = "extra")]
impl<'a> ContextEmailFacade<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn send(
        &self,
        server: &str,
        user: &str,
        pass: &str,
        from: &str,
        to: &str,
        subject: &str,
        body: &str,
    ) -> MustAuditAs<Result<()>> {
        let server = server.to_owned();
        let user = user.to_owned();
        let pass = pass.to_owned();
        let from = from.to_owned();
        let to = to.to_owned();
        let subject = subject.to_owned();
        let body = body.to_owned();

        MustAuditAs::new(move |_desc| {
            EmailTool::new().send(&server, &user, &pass, &from, &to, &subject, &body)
        })
    }
}
