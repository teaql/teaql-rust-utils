use crate::macros::*;

use teaql_tool_extra::template::TemplateTool;
use teaql_tool_core::Result;

define_context_facade!("extra", template, ContextTemplateExt, ContextTemplateFacade);

#[cfg(feature = "extra")]
impl<'a> ContextTemplateFacade<'a> {
    delegate_res_comment! { TemplateTool::new(),
        fn render(&self, template: &str, ctx_json: &str) -> String
    }
}
