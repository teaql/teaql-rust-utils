use crate::macros::*;

use teaql_tool_extra::html::HtmlTool;
use teaql_tool_core::Result;

define_context_facade!("extra", html, ContextHtmlExt, ContextHtmlFacade);

#[cfg(feature = "extra")]
impl<'a> ContextHtmlFacade<'a> {
    delegate_res_comment! { HtmlTool::new(),
        fn select_text(&self, html_str: &str, selector_str: &str) -> Vec<String>;
        fn select_attr(&self, html_str: &str, selector_str: &str, attr: &str) -> Vec<String>
    }
}
